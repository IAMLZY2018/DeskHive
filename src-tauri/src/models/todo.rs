use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    #[serde(default = "generate_uuid")] // 为兼容旧数据，设为默认值
    pub id: String, // 添加唯一ID
    pub text: String,
    pub completed: bool,
    pub created_at: i64, // Unix时间戳（秒）
    #[serde(default)] // 为了兼容旧数据，设为默认值
    pub deadline: Option<i64>, // 截止时间，Unix时间戳（秒），可选
}

// 生成UUID的函数
fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

#[derive(Serialize, Deserialize)]
pub struct TodoData {
    pub pending_todos: Vec<Todo>,
    pub completed_todos: Vec<Todo>,
}