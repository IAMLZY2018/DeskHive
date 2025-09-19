use std::fs;
use std::path::PathBuf;
use serde_json;
use tauri::Manager;

use crate::models::{Todo, TodoData};

// 获取数据目录路径
fn get_data_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    // 使用文档目录而不是应用数据目录，这样重装或更新应用时数据不会丢失
    let document_dir = app.path().document_dir()
        .map_err(|e| format!("获取用户文档目录失败: {}", e))?;
    
    // 创建DeskHive专用的数据目录
    let data_dir = document_dir.join("DeskHive").join("data");
    
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
        // 如果文件不存在，返回空数据
        return Ok(TodoData {
            pending_todos: vec![],
            completed_todos: vec![],
        });
    }
    
    let json_data = fs::read_to_string(&file_path)
        .map_err(|e| format!("读取文件失败: {}", e))?;
    
    let todo_data: TodoData = serde_json::from_str(&json_data)
        .map_err(|e| format!("解析JSON失败: {}", e))?;
    
    Ok(todo_data)
}

// Tauri 命令：更新todo文本内容
#[tauri::command]
pub async fn update_todo_text(
    app: tauri::AppHandle,
    todo_id: String,
    is_completed: bool,
    new_text: String
) -> Result<(), String> {
    println!("准备更新任务文本: id='{}', completed={}, new_text='{}'?", 
        todo_id, is_completed, new_text);
    
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
        println!("找到对应的todo项，更新文本");
        println!("更新前的文本: '{}'", todo.text);
        todo.text = new_text;
        println!("更新后的文本: '{}'", todo.text);
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
        
        println!("任务文本更新成功");
        Ok(())
    } else {
        let error_msg = format!("未找到指定的todo项: id='{}', completed={}", todo_id, is_completed);
        println!("{}", error_msg);
        Err(error_msg)
    }
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