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
    pub completed_at: Option<i64>, // 完成时间，Unix时间戳（秒），可选
    #[serde(default)] // 为了兼容旧数据，设为默认值
    pub deadline: Option<i64>, // 截止时间，Unix时间戳（秒），可选
    #[serde(default)] // 为了兼容旧数据，设为默认值
    pub order: i32, // 在分组内的排序
    #[serde(default = "default_group_id")] // 为兼容旧数据，设为默认分组
    pub group_id: String, // 所属分组ID
}

// 默认分组ID
fn default_group_id() -> String {
    "default".to_string()
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


#[derive(Serialize, Deserialize, Clone)]
pub struct TodoGroup {
    pub id: String,
    pub name: String,
    pub order: i32,
    pub collapsed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct TodoDataWithGroups {
    pub todos: Vec<Todo>,
}

#[derive(Serialize, Deserialize)]
pub struct GroupData {
    pub groups: Vec<TodoGroup>,
}
