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
    #[serde(default = "default_priority_color")]
    pub priority_color: String,
    #[serde(default = "default_window_level")]
    pub window_level: String, // "normal" | "always_on_top" | "always_on_bottom"
    #[serde(default = "default_timeline_deadline_priority")]
    pub timeline_deadline_priority: bool, // 时间轴视图是否优先使用截止时间
    #[serde(default = "default_enable_deadline_notification")]
    pub enable_deadline_notification: bool, // 是否启用截止时间通知
    #[serde(default = "default_notification_minutes_before")]
    pub notification_minutes_before: u32, // 提前多少分钟通知
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
            priority_color: "#FF9800".to_string(),
            window_level: "always_on_bottom".to_string(),
            timeline_deadline_priority: true,
            enable_deadline_notification: false,
            notification_minutes_before: 30,
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

pub fn default_priority_color() -> String {
    "#FF9800".to_string()
}

pub fn default_window_level() -> String {
    "always_on_bottom".to_string()
}

pub fn default_timeline_deadline_priority() -> bool {
    true
}

pub fn default_enable_deadline_notification() -> bool {
    false
}

pub fn default_notification_minutes_before() -> u32 {
    30
}