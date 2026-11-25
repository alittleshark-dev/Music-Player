use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, glib, Box, Label,
    Button, Popover, ListBox, ListBoxRow, ScrolledWindow,
};
use gtk4_layer_shell::{Layer, LayerShell, Edge};
use std::rc::Rc;
use std::cell::RefCell;
use std::path::PathBuf;
use walkdir::WalkDir;

mod load_conf;
mod player;
mod load_css;

const APP_ID: &str = "com.alittleshark.musicPlayer";

fn main() -> glib::ExitCode {
    let config = load_conf::load_or_create_config();
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(move |app| {
        // GTK 已完成初始化，此时 Display 可用；对单窗口应用 activate 只触发一次
        load_css::load_css(&config);
        build_ui(app, &config);
    });

    app.run()
}

fn build_ui(app: &Application, config: &load_conf::AppConfig) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Niri Music Player")
        .default_width(320)
        .default_height(400)
        .build();

    window.init_layer_shell();
    window.set_namespace(Some("music-player"));
    window.set_layer(Layer::Bottom);

    window.set_anchor(Edge::Top, true);
    window.set_anchor(Edge::Right, true);
    window.set_margin(Edge::Top, config.layout.top.unwrap_or(20));
    window.set_margin(Edge::Right, config.layout.left.unwrap_or(20));
    window.set_exclusive_zone(-1);

    let vbox = Box::new(gtk::Orientation::Vertical, 10);
    vbox.add_css_class("player-box");

    // 标题
    let title_label = Label::new(Some("等待播放..."));
    title_label.add_css_class("title-label");
    vbox.append(&title_label);

    // 控制栏
    let hbox = Box::new(gtk::Orientation::Horizontal, 10);
    hbox.set_halign(gtk::Align::Center);
    let btn_prev = Button::from_icon_name("media-skip-backward");
    let btn_play = Button::from_icon_name("media-playback-start");
    let btn_next = Button::from_icon_name("media-skip-forward");
    let btn_loop = Button::from_icon_name("media-playlist-repeat");
    btn_loop.set_tooltip_text(Some("循环模式: 关闭"));
    let btn_menu = Button::from_icon_name("view-list-symbolic");
    btn_menu.add_css_class("menu-btn");
    hbox.append(&btn_prev);
    hbox.append(&btn_play);
    hbox.append(&btn_next);
    hbox.append(&btn_loop);
    hbox.append(&btn_menu);
    vbox.append(&hbox);

    let audio_player = Rc::new(RefCell::new(player::AudioPlayer::new()));

    let music_dir = dirs::audio_dir().unwrap_or_else(|| dirs::home_dir().unwrap().join("Music"));
    println!("正在扫描音乐目录: {:?}", music_dir);

    let mut song_paths: Vec<PathBuf> = Vec::new();
    let mut song_names: Vec<String> = Vec::new();

    for entry in WalkDir::new(&music_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "mp3" || ext == "flac" || ext == "wav") {
            let name = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
            song_names.push(name);
            song_paths.push(path.to_path_buf());
        }
    }
    println!("找到 {} 首歌", song_paths.len());

    let popover = Popover::new();
    popover.set_has_arrow(false);
    popover.set_size_request(320, 400);
    popover.add_css_class("popover");

    let list_box = ListBox::new();
    list_box.add_css_class("popover_list");
    list_box.set_selection_mode(gtk::SelectionMode::Single);

    for name in &song_names {
        let row = ListBoxRow::new();
        let label = Label::new(Some(name));
        label.set_halign(gtk::Align::Start);
        label.set_margin_top(8);
        label.set_margin_bottom(8);
        label.set_margin_start(12);
        row.set_child(Some(&label));
        list_box.append(&row);
    }

    let scroll = ScrolledWindow::new();
    scroll.set_child(Some(&list_box));
    scroll.set_vexpand(true);
    popover.set_child(Some(&scroll));
    popover.set_parent(&btn_menu);

    // 点击菜单显示
    let popover_clone = popover.clone();
    btn_menu.connect_clicked(move |_| {
        popover_clone.popup();
    });

    // 循环模式切换
    let audio_player_clone_loop = audio_player.clone();
    let btn_loop_clone = btn_loop.clone();
    btn_loop.connect_clicked(move |_| {
        let mut p = audio_player_clone_loop.borrow_mut();
        let next = match p.loop_mode() {
            player::LoopMode::None => player::LoopMode::One,
            player::LoopMode::One => player::LoopMode::None,
        };
        p.set_loop_mode(next);
        drop(p);  // 释放借用，后面只操作 UI

        let (icon, tip) = match next {
            player::LoopMode::None => ("media-playlist-repeat", "循环模式: 关闭"),
            player::LoopMode::One => ("media-playlist-repeat-symbolic", "循环模式: 单曲循环"),
        };
        btn_loop_clone.set_icon_name(icon);
        btn_loop_clone.set_tooltip_text(Some(tip));
        println!(">>> 循环模式: {:?}", next);
    });

    let title_label_clone = title_label.clone();
    let audio_player_clone = audio_player.clone();
    let song_paths_rc = Rc::new(song_paths);
    let song_names_rc = Rc::new(song_names);

    list_box.connect_row_activated(move |_, row| {
        let idx = row.index() as usize;
        if idx >= song_paths_rc.len() {
            return;
        }

        let path = &song_paths_rc[idx];
        println!(">>> 准备播放: {:?}", path);

        // 借用范围尽量小：play 返回后立即释放 borrow_mut
        let result = {
            let mut p = audio_player_clone.borrow_mut();
            p.play(path)
        };

        match result {
            Ok(()) => title_label_clone.set_text(&song_names_rc[idx]),
            Err(e) => eprintln!("播放失败: {}", e),
        }
    });

    let audio_player_clone2 = audio_player.clone();
    btn_play.connect_clicked(move |_| {
        let player = audio_player_clone2.borrow();
        player.toggle_pause();
        println!(">>> 切换播放/暂停");
    });

    window.set_child(Some(&vbox));
    window.present();
}
