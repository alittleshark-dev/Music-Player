// src/load_conf.rs
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Default)]
pub struct AppConfig {
    #[serde(default)]
    pub layout: LayoutConfig,

    #[serde(default)]
    pub size: SizeConfig,

    #[serde(default)]
    pub css: CssConfig,
}

#[derive(Debug, Deserialize, Default)]
pub struct SizeConfig {
    pub width: Option<i32>,
    pub height: Option<i32>,
}

// [layout] 块
#[derive(Debug, Deserialize, Default)]
pub struct LayoutConfig {
    pub top: Option<i32>,
    pub right: Option<i32>,
    pub bottom: Option<i32>,
    pub left: Option<i32>,
}

// [css] 块
#[derive(Debug, Deserialize)]
pub struct CssConfig {
    pub path: String,
}

// 默认值
impl Default for CssConfig {
    fn default() -> Self {
        Self {
            path: "~/.config/music-player/style.css".to_string(),
        }
    }
}

// 获取配置文件路径
pub fn get_config_path() -> PathBuf {
    let mut path = dirs::config_dir().expect("无法找到配置目录");
    path.push("music-player");
    path.push("config.toml"); // 改用 toml 后缀
    path
}

pub fn load_or_create_config() -> AppConfig {
    let config_path = get_config_path();
    
    if config_path.exists() {
        if let Ok(content) = fs::read_to_string(&config_path) {
            // 解析 TOML
            match toml::from_str(&content) {
                Ok(config) => return config,
                Err(e) => eprintln!("配置文件解析失败，使用默认配置: {}", e),
            }
        }
    } else {
        // 如果文件不存在，生成一份模版
        if let Some(parent) = config_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let home = dirs::home_dir().expect("找不到家目录");
        let css_path = home.join(".config/music-player/style.css");

        let default_toml = format!(r#"
[layout]
top = 10
left = 10

[css]
path = "{}"
"#, css_path.display());
        let _ = fs::write(&config_path, default_toml);
        println!("已创建默认配置文件: {:?}", config_path);
    }

    AppConfig::default()
}
