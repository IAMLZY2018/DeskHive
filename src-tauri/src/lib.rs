use std::sync::atomic::{AtomicBool, Ordering};
use std::path::PathBuf;
use std::fs;
use serde::{Deserialize, Serialize};
use tauri::Manager;

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
    .decorations(true)
    .always_on_top(false)
    .skip_taskbar(false)
    .build()
    .map_err(|e| format!("创建设置窗口失败: {}", e))?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_settings_window, save_todo_data, load_todo_data])
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
                // 使用更简单的方法优化启动体验
                #[cfg(target_os = "windows")]
                {
                    // 先隐藏窗口
                    let _ = window.hide();

                    // 创建一个线程，短暂延迟后显示窗口
                    let win_load = window.clone();
                    std::thread::spawn(move || {
                        // 等待一小段时间，让前端有时间渲染
                        std::thread::sleep(std::time::Duration::from_millis(300));
                        // 显示窗口
                        let _ = win_load.show();
                    });
                }

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
