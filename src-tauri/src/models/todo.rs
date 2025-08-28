use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    pub text: String,
    pub completed: bool,
    pub created_at: i64, // Unix时间戳（秒）
    #[serde(default)] // 为了兼容旧数据，设为默认值
    pub deadline: Option<i64>, // 截止时间，Unix时间戳（秒），可选
}

#[derive(Serialize, Deserialize)]
pub struct TodoData {
    pub pending_todos: Vec<Todo>,
    pub completed_todos: Vec<Todo>,
}