use std::sync::atomic::{AtomicBool, Ordering};
use std::path::PathBuf;
use std::fs;
use serde::{Deserialize, Serialize};
use tauri::Manager;

#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{SetWindowLongPtrW, GetWindowLongPtrW, SetLayeredWindowAttributes, GWL_EXSTYLE, WS_EX_LAYERED, LWA_ALPHA};
#[cfg(target_os = "windows")]
use windows::Win32::Foundation::{HWND, COLORREF};
#[cfg(target_os = "windows")]
use raw_window_handle::{HasWindowHandle, RawWindowHandle};

// 创建一个全局变量来跟踪Win+D状态
static WIN_D_PRESSED: AtomicBool = AtomicBool::new(false);

// Todo数据结构
#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    text: String,
    completed: bool,
}

#[derive(Serialize, Deserialize)]
struct TodoData {
    pending_todos: Vec<Todo>,
    completed_todos: Vec<Todo>,
}

// 应用设置结构
#[derive(Serialize, Deserialize, Clone)]
struct AppSettings {
    opacity: f64,
    always_on_top: bool,
    auto_show: bool,
    minimize_to_tray: bool,
    hotkey: String,
}

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
async fn save_todo_data(app: tauri::AppHandle, pending_todos: Vec<Todo>, completed_todos: Vec<Todo>) -> Result<(), String> {
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
async fn load_todo_data(app: tauri::AppHandle) -> Result<TodoData, String> {
    let data_dir = get_data_dir(&app)?;
    let file_path = data_dir.join("todo_list.json");
    
    if !file_path.exists() {
        // 如果文件不存在，返回默认数据
        return Ok(TodoData {
            pending_todos: vec![
                Todo { text: "学习SpringBoot3.5".to_string(), completed: false },
                Todo { text: "测试部署到服务器".to_string(), completed: false },
            ],
            completed_todos: vec![
                Todo { text: "完成UI设计".to_string(), completed: true },
            ],
        });
    }
    
    let json_data = fs::read_to_string(&file_path)
        .map_err(|e| format!("读取文件失败: {}", e))?;
    
    let todo_data: TodoData = serde_json::from_str(&json_data)
        .map_err(|e| format!("解析JSON失败: {}", e))?;
    
    Ok(todo_data)
}

// Tauri 命令：关闭设置窗口
#[tauri::command]
async fn close_settings_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("settings") {
        window.close().map_err(|e| format!("关闭设置窗口失败: {}", e))?;
    }
    Ok(())
}

// Tauri 命令：保存应用设置
#[tauri::command]
async fn save_app_settings(app: tauri::AppHandle, settings: AppSettings) -> Result<(), String> {
    let data_dir = get_data_dir(&app)?;
    let file_path = data_dir.join("app_settings.json");
    
    let json_data = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("序列化设置失败: {}", e))?;
    
    fs::write(&file_path, json_data)
        .map_err(|e| format!("写入设置文件失败: {}", e))?;
    
    // 应用设置到主窗口
    if let Some(main_window) = app.get_webview_window("main") {
        // 设置透明度
        let _ = set_window_opacity(&main_window, settings.opacity);
        
        // 设置置顶状态
        let _ = main_window.set_always_on_top(settings.always_on_top);
    }
    
    Ok(())
}

// Tauri 命令：加载应用设置
#[tauri::command]
async fn load_app_settings(app: tauri::AppHandle) -> Result<AppSettings, String> {
    let data_dir = get_data_dir(&app)?;
    let file_path = data_dir.join("app_settings.json");
    
    if !file_path.exists() {
        // 如果文件不存在，返回默认设置
        return Ok(AppSettings {
            opacity: 0.95,
            always_on_top: false,
            auto_show: true,
            minimize_to_tray: true,
            hotkey: "ctrl+shift+t".to_string(),
        });
    }
    
    let json_data = fs::read_to_string(&file_path)
        .map_err(|e| format!("读取设置文件失败: {}", e))?;
    
    let settings: AppSettings = serde_json::from_str(&json_data)
        .map_err(|e| format!("解析设置JSON失败: {}", e))?;
    
    Ok(settings)
}

// Tauri 命令：应用透明度设置
#[tauri::command]
async fn apply_opacity(app: tauri::AppHandle, opacity: f64) -> Result<(), String> {
    if let Some(main_window) = app.get_webview_window("main") {
        set_window_opacity(&main_window, opacity)?;
    }
    Ok(())
}

// 设置窗口透明度的辅助函数
#[cfg(target_os = "windows")]
fn set_window_opacity(window: &tauri::WebviewWindow, opacity: f64) -> Result<(), String> {
    let window_handle = window.window_handle()
        .map_err(|e| format!("获取窗口句柄失败: {}", e))?;
    
    let raw_handle = window_handle.as_raw();
    
    if let RawWindowHandle::Win32(handle) = raw_handle {
        unsafe {
            let hwnd = HWND(handle.hwnd.get() as isize);
            
            // 获取当前窗口样式
            let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
            
            // 添加分层窗口样式
            SetWindowLongPtrW(hwnd, GWL_EXSTYLE, ex_style | WS_EX_LAYERED.0 as isize);
            
            // 设置透明度 (0-255)
            let alpha = (opacity * 255.0) as u8;
            let result = SetLayeredWindowAttributes(hwnd, COLORREF(0), alpha, LWA_ALPHA);
            
            if !result.as_bool() {
                return Err("设置透明度失败".to_string());
            }
        }
    }
    
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn set_window_opacity(_window: &tauri::WebviewWindow, _opacity: f64) -> Result<(), String> {
    // 在非Windows平台上，暂时不支持透明度设置
    Ok(())
}

// Tauri 命令：打开设置窗口
#[tauri::command]
async fn open_settings_window(app: tauri::AppHandle) -> Result<(), String> {
    // 检查设置窗口是否已经存在
    if let Some(window) = app.get_webview_window("settings") {
        // 如果窗口已存在，直接显示并聚焦
        let _ = window.show();
        let _ = window.set_focus();
        return Ok(());
    }

    // 使用 Tauri 2.x 的 API 创建新窗口
    let _settings_window = tauri::WebviewWindowBuilder::new(
        &app,
        "settings",
        tauri::WebviewUrl::App("settings.html".into()),
    )
    .title("设置")
    .inner_size(800.0, 600.0)
    .min_inner_size(800.0, 600.0)
    .center()
    .resizable(false)
    .decorations(false)
    .always_on_top(false)
    .skip_taskbar(false)
    .build()
    .map_err(|e| format!("创建设置窗口失败: {}", e))?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            open_settings_window, 
            close_settings_window, 
            save_todo_data, 
            load_todo_data,
            save_app_settings,
            load_app_settings,
            apply_opacity
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // 获取主窗口
            if let Some(window) = app.get_webview_window("main") {
                // 加载并应用保存的设置
                let app_handle = app.handle().clone();
                tauri::async_runtime::spawn(async move {
                    if let Ok(settings) = load_app_settings(app_handle.clone()).await {
                        if let Some(main_window) = app_handle.get_webview_window("main") {
                            // 应用透明度设置
                            let _ = set_window_opacity(&main_window, settings.opacity);
                            // 应用置顶设置
                            let _ = main_window.set_always_on_top(settings.always_on_top);
                        }
                    }
                });
                
                // 简化启动逻辑，直接显示窗口
                let _ = window.show();
                let _ = window.set_focus();

                // 在Tauri 2.x中处理窗口事件
                #[cfg(target_os = "windows")]
                {
                    let win_handle = window.clone();

                    // 监听窗口事件
                    window.on_window_event(move |event| {
                        match event {
                            // 检测窗口可见性变化
                            tauri::WindowEvent::Focused(focused) => {
                                if !focused {
                                    // 可能是Win+D被触发了或窗口失去焦点
                                    WIN_D_PRESSED.store(true, Ordering::SeqCst);

                                    // 设置一个短暂延迟后尝试恢复窗口
                                    let win = win_handle.clone();
                                    std::thread::spawn(move || {
                                        std::thread::sleep(std::time::Duration::from_millis(100));
                                        if WIN_D_PRESSED.load(Ordering::SeqCst) {
                                            // 尝试恢复窗口，但不强制获取焦点
                                            let _ = win.show();
                                            WIN_D_PRESSED.store(false, Ordering::SeqCst);
                                        }
                                    });
                                }
                            }
                            // 检测窗口隐藏事件
                            tauri::WindowEvent::CloseRequested { .. } => {
                                // 不做任何处理，让窗口正常关闭
                            }
                            // 其他事件
                            _ => {}
                        }
                    });

                    // 添加定时器，但只在检测到Win+D时才尝试恢复窗口
                    let win_handle2 = window.clone();
                    std::thread::spawn(move || {
                        // 不需要初始等待，直接开始监听
                        loop {
                            std::thread::sleep(std::time::Duration::from_millis(800)); // 使用适中的检查间隔

                            // 只有当WIN_D_PRESSED为true时才检查窗口可见性
                            if WIN_D_PRESSED.load(Ordering::SeqCst) {
                                // 检查窗口是否可见
                                match win_handle2.is_visible() {
                                    Ok(visible) => {
                                        if !visible {
                                            // 窗口不可见，尝试恢复，但不强制获取焦点
                                            let _ = win_handle2.show();
                                            // 恢复后重置标志
                                            WIN_D_PRESSED.store(false, Ordering::SeqCst);
                                        } else {
                                            // 窗口已经可见，重置标志
                                            WIN_D_PRESSED.store(false, Ordering::SeqCst);
                                        }
                                    }
                                    Err(_) => {
                                        // 窗口可能已关闭，重置标志
                                        WIN_D_PRESSED.store(false, Ordering::SeqCst);
                                    }
                                }
                            }

                            // 检查窗口是否还存在
                            if win_handle2.is_visible().is_err() {
                                // 窗口可能已关闭，退出循环
                                break;
                            }
                        }
                    });
                }
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
