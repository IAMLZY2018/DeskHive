use std::fs;
use std::path::PathBuf;
use serde_json;
use tauri::{Manager, ApiError};

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
