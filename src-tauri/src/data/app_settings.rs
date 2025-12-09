use std::fs;
use serde_json;
use tauri::{Manager, Emitter};

use crate::models::AppSettings;
use crate::system::auto_start::set_auto_start;
use crate::window::opacity::set_window_opacity;

// 获取数据目录路径
fn get_data_dir(app: &tauri::AppHandle) -> Result<std::path::PathBuf, String> {
    // 使用文档目录而不是应用数据目录，这样重装或更新应用时数据不会丢失
    let document_dir = app.path().document_dir()
        .map_err(|e| format!("获取用户文档目录失败: {}", e))?;
    
    // 创建DeskHive专用的数据目录
    let data_dir = document_dir.join("DeskHive").join("data");
    
    // 确保data目录存在
    if !data_dir.exists() {
        std::fs::create_dir_all(&data_dir)
            .map_err(|e| format!("创建data目录失败: {}", e))?;
    }
    
    Ok(data_dir)
}

// Tauri 命令：保存应用设置
#[tauri::command]
pub async fn save_app_settings(app: tauri::AppHandle, settings: AppSettings) -> Result<(), String> {
    let data_dir = get_data_dir(&app)?;
    let file_path = data_dir.join("app_settings.json");
    
    // 处理开机自启动设置
    if let Ok(old_settings) = crate::data::load_app_settings(app.clone()).await {
        // 如果开机自启动设置发生了变化，则更新系统设置
        if old_settings.auto_start != settings.auto_start {
            set_auto_start(&app, settings.auto_start)?;
        }
    } else {
        // 如果无法加载旧设置，直接应用新设置
        set_auto_start(&app, settings.auto_start)?;
    }
    
    let json_data = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("序列化设置失败: {}", e))?;
    
    fs::write(&file_path, json_data)
        .map_err(|e| format!("写入设置文件失败: {}", e))?;
    
    // 应用设置到主窗口（设置窗口保持不透明）
    if let Some(main_window) = app.get_webview_window("main") {
        // 设置透明度（只应用于主窗口）
        let _ = set_window_opacity(&main_window, settings.opacity);
        
        // 设置窗口层级 - 与托盘菜单功能保持一致
        match settings.window_level.as_str() {
            "always_on_top" => {
                // 置于顶层：使用 set_always_on_top
                let _ = main_window.set_always_on_top(true);
            },
            "always_on_bottom" => {
                // 置于桌面：使用 Windows API 将窗口置于底层
                #[cfg(target_os = "windows")]
                {
                    use windows::Win32::Foundation::HWND;
                    use windows::Win32::UI::WindowsAndMessaging::{
                        SetWindowPos, HWND_BOTTOM, SWP_NOMOVE, SWP_NOSIZE, SWP_NOACTIVATE
                    };
                    
                    if let Ok(hwnd) = main_window.hwnd() {
                        unsafe {
                            let window_hwnd = HWND(hwnd.0 as _);
                            let _ = SetWindowPos(
                                window_hwnd,
                                HWND_BOTTOM,
                                0, 0, 0, 0,
                                SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE
                            );
                        }
                    }
                }
                // 确保不是置顶状态
                let _ = main_window.set_always_on_top(false);
            },
            _ => {
                // 默认：取消置顶
                let _ = main_window.set_always_on_top(false);
            }
        }
        
        // 通知前端更新拖动设置
        let _ = main_window.emit("drag-setting-changed", settings.disable_drag);
    }
    
    Ok(())
}

// Tauri 命令：加载应用设置
#[tauri::command]
pub async fn load_app_settings(app: tauri::AppHandle) -> Result<AppSettings, String> {
    let data_dir = get_data_dir(&app)?;
    let file_path = data_dir.join("app_settings.json");
    
    if !file_path.exists() {
        // 如果文件不存在，返回默认设置
        return Ok(AppSettings {
            opacity: 1.0,
            disable_drag: false,
            auto_show: true,
            minimize_to_tray: true,
            auto_start: false,
            silent_start: false,
            theme: "light".to_string(),
            priority_color: "#FF9800".to_string(),
            window_level: "always_on_bottom".to_string(),
        });
    }
    
    let json_data = std::fs::read_to_string(&file_path)
        .map_err(|e| format!("读取设置文件失败: {}", e))?;
    
    let settings: AppSettings = serde_json::from_str(&json_data)
        .map_err(|e| format!("解析设置JSON失败: {}", e))?;
    
    Ok(settings)
}

// Tauri 命令：应用透明度设置（只应用于主窗口）
#[tauri::command]
pub async fn apply_opacity(app: tauri::AppHandle, opacity: f64) -> Result<(), String> {
    // 只对主窗口应用透明度，设置窗口保持不透明
    if let Some(main_window) = app.get_webview_window("main") {
        set_window_opacity(&main_window, opacity)?;
    }
    Ok(())
}