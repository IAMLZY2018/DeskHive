use std::fs;
use serde_json;
use tauri::{Manager};

use crate::models::WindowPosition;

// 获取数据目录路径
fn get_data_dir(app: &tauri::AppHandle) -> Result<std::path::PathBuf, String> {
    let app_dir = app.path().app_data_dir()
        .map_err(|e| format!("获取应用数据目录失败: {}", e))?;
    
    let data_dir = app_dir.join("data");
    
    // 确保data目录存在
    if !data_dir.exists() {
        std::fs::create_dir_all(&data_dir)
            .map_err(|e| format!("创建data目录失败: {}", e))?;
    }
    
    Ok(data_dir)
}

// Tauri 命令：保存窗口位置
#[tauri::command]
pub async fn save_window_position(app: tauri::AppHandle, x: i32, y: i32) -> Result<(), String> {
    let data_dir = get_data_dir(&app)?;
    let file_path = data_dir.join("window_position.json");
    
    let position = WindowPosition { x, y };
    
    let json_data = serde_json::to_string_pretty(&position)
        .map_err(|e| format!("序列化窗口位置失败: {}", e))?;
    
    fs::write(&file_path, json_data)
        .map_err(|e| format!("写入窗口位置文件失败: {}", e))?;
    
    Ok(())
}

// Tauri 命令：加载窗口位置
#[tauri::command]
pub async fn load_window_position(app: tauri::AppHandle) -> Result<Option<WindowPosition>, String> {
    let data_dir = get_data_dir(&app)?;
    let file_path = data_dir.join("window_position.json");
    
    if !file_path.exists() {
        return Ok(None);
    }
    
    let json_data = fs::read_to_string(&file_path)
        .map_err(|e| format!("读取窗口位置文件失败: {}", e))?;
    
    let position: WindowPosition = serde_json::from_str(&json_data)
        .map_err(|e| format!("解析窗口位置JSON失败: {}", e))?;
    
    Ok(Some(position))
}