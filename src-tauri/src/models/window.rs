use serde::{Deserialize, Serialize};

// 窗口位置结构
#[derive(Serialize, Deserialize, Clone)]
pub struct WindowPosition {
    pub x: i32,
    pub y: i32,
}