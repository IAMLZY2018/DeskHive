use std::fs;
use std::path::PathBuf;
use serde_json;
use tauri::Manager;
use uuid::Uuid;

use crate::models::{Todo, TodoData};

// 获取数据目录路径
fn get_data_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let app_dir = app.path().app_data_dir()
        .map_err(|e| format!("获取应用数据目录失败: {}", e))?;
    
    let data_dir = app_dir.join("data");
    
    // 确保data目录存在
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)
            .map_err(|e| format!("创建data目录失败: {}", e))?;
    }
    
    Ok(data_dir)
}

// Tauri 命令：保存todo数据
#[tauri::command]
pub async fn save_todo_data(app: tauri::AppHandle, pending_todos: Vec<Todo>, completed_todos: Vec<Todo>) -> Result<(), String> {
    let data_dir = get_data_dir(&app)?;
    let file_path = data_dir.join("todo_list.json");
    
    let todo_data = TodoData {
        pending_todos,
        completed_todos,
    };
    
    let json_data = serde_json::to_string_pretty(&todo_data)
        .map_err(|e| format!("序列化数据失败: {}", e))?;
    
    fs::write(&file_path, json_data)
        .map_err(|e| format!("写入文件失败: {}", e))?;
    
    Ok(())
}

// Tauri 命令：加载todo数据
#[tauri::command]
pub async fn load_todo_data(app: tauri::AppHandle) -> Result<TodoData, String> {
    let data_dir = get_data_dir(&app)?;
    let file_path = data_dir.join("todo_list.json");
    
    if !file_path.exists() {
        // 如果文件不存在，返回默认数据
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
            
        return Ok(TodoData {
            pending_todos: vec![
                Todo { 
                    id: Uuid::new_v4().to_string(),
                    text: "欢迎使用DeskHive桌面助手".to_string(),
                    completed: false,
                    created_at: now - 3600, // 1小时前创建
                    deadline: None, // 无截止时间
                },
                Todo { 
                    id: Uuid::new_v4().to_string(),
                    text: "测试天数指示器功能 - 3天前创建".to_string(),
                    completed: false,
                    created_at: now - (3 * 24 * 3600), // 3天前创建
                    deadline: Some(now + (2 * 24 * 3600)), // 设置2天后截止，用于测试倒计时
                },
                Todo { 
                    id: Uuid::new_v4().to_string(),
                    text: "测试天数指示器功能 - 7天前创建".to_string(),
                    completed: false,
                    created_at: now - (7 * 24 * 3600), // 7天前创建
                    deadline: None, // 无截止时间
                },
            ],
            completed_todos: vec![
                Todo { 
                    id: Uuid::new_v4().to_string(),
                    text: "已完成的会收纳到这里，可以双击删除哦~".to_string(),
                    completed: true,
                    created_at: now - (5 * 24 * 3600), // 5天前创建
                    deadline: Some(now - (1 * 24 * 3600)), // 设置1天前截止（已过期），用于测试过期显示
                },
            ],
        });
    }
    
    let json_data = fs::read_to_string(&file_path)
        .map_err(|e| format!("读取文件失败: {}", e))?;
    
    let todo_data: TodoData = serde_json::from_str(&json_data)
        .map_err(|e| format!("解析JSON失败: {}", e))?;
    
    Ok(todo_data)
}

// Tauri 命令：设置todo截止时间
#[tauri::command]
pub async fn set_todo_deadline(
    app: tauri::AppHandle, 
    todo_id: String,  // 使用ID而不是文本
    is_completed: bool,
    deadline: Option<i64>
) -> Result<(), String> {
    println!("准备设置截止时间: id='{}', completed={}, deadline={:?}", 
        todo_id, is_completed, deadline);
    
    // 先加载当前数据
    let mut todo_data = load_todo_data(app.clone()).await?;
    
    println!("加载的数据: pending_count={}, completed_count={}", 
        todo_data.pending_todos.len(), todo_data.completed_todos.len());
    
    // 查找并更新对应的todo项
    let found = if is_completed {
        // 在已完成列表中查找
        println!("在已完成列表中查找");
        for (i, todo) in todo_data.completed_todos.iter().enumerate() {
            println!("  [{}]: '{}' (id: {})", i, todo.text, todo.id);
        }
        todo_data.completed_todos.iter_mut()
            .find(|todo| todo.id == todo_id)
    } else {
        // 在待完成列表中查找
        println!("在待完成列表中查找");
        for (i, todo) in todo_data.pending_todos.iter().enumerate() {
            println!("  [{}]: '{}' (id: {})", i, todo.text, todo.id);
        }
        todo_data.pending_todos.iter_mut()
            .find(|todo| todo.id == todo_id)
    };
    
    if let Some(todo) = found {
        println!("找到对应的todo项，更新deadline");
        println!("更新前的deadline: {:?}", todo.deadline);
        todo.deadline = deadline;
        println!("更新后的deadline: {:?}", todo.deadline);
        // 保存更新后的数据
        println!("准备保存数据到文件");
        let save_result = save_todo_data(app, todo_data.pending_todos.clone(), todo_data.completed_todos.clone()).await;
        
        match &save_result {
            Ok(_) => println!("数据保存成功"),
            Err(e) => println!("数据保存失败: {}", e),
        }
        
        if save_result.is_err() {
            return save_result;
        }
        
        // 根据deadline值提供不同的成功消息
        if deadline.is_some() {
            println!("截止时间设置成功");
        } else {
            println!("截止时间移除成功");
        }
        Ok(())
    } else {
        let error_msg = format!("未找到指定的todo项: id='{}', completed={}", todo_id, is_completed);
        println!("{}", error_msg);
        Err(error_msg)
    }
}