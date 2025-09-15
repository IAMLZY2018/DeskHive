use serde::{Deserialize, Serialize};

// 应用设置结构
#[derive(Serialize, Deserialize, Clone)]
pub struct AppSettings {
    pub opacity: f64,
    pub disable_drag: bool,
    #[serde(default = "default_auto_show")]
    pub auto_show: bool,
    #[serde(default = "default_minimize_to_tray")]
    pub minimize_to_tray: bool,
    #[serde(default = "default_auto_start")]
    pub auto_start: bool,
    #[serde(default = "default_silent_start")]
    pub silent_start: bool,
    #[serde(default = "default_theme")]
    pub theme: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            opacity: 1.0,
            disable_drag: false,
            auto_show: true,
            minimize_to_tray: true,
            auto_start: false,
            silent_start: false,
            theme: "light".to_string(),
        }
    }
}

// 默认值函数
pub fn default_auto_show() -> bool {
    true
}

pub fn default_minimize_to_tray() -> bool {
    true
}

pub fn default_auto_start() -> bool {
    false
}

pub fn default_silent_start() -> bool {
    false
}

pub fn default_theme() -> String {
    "light".to_string()
}