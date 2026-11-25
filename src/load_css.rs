use crate::load_conf;
use std::path::PathBuf;

pub fn load_css(config: &load_conf::AppConfig) {
    let provider = gtk::CssProvider::new();
    let raw_css = &config.css.path;
    let css_path = if let Some(rest) = raw_css.strip_prefix("~/") {
        dirs::home_dir()
            .map(|h| h.join(rest))
            .unwrap_or_else(|| PathBuf::from(raw_css))
    } else {
        PathBuf::from(raw_css)
    };

    if css_path.exists() {
        provider.load_from_path(css_path.to_str().unwrap_or(""));
        gtk::style_context_add_provider_for_display(
            &gtk::gdk::Display::default().expect("无法连接显示器"),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    } else {
        eprintln!("CSS 文件不存在，跳过样式加载: {:?}", css_path);
    }
}