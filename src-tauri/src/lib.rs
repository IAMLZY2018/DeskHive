// 防止在Windows发布版本中出现额外的控制台窗口，请勿删除！！
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(target_os = "windows")]
use windows::Win32::System::Threading::CreateMutexW;
#[cfg(target_os = "windows")]
use windows::Win32::Foundation::CloseHandle;
#[cfg(target_os = "windows")]
use std::ffi::OsStr;
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStrExt;

use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{Manager, Emitter};

// 模块声明
mod models;
mod data;
mod window;
mod system;
mod utils;
mod notification;

// 重新导出需要的类型和函数
use data::{
    save_todo_data, 
    load_todo_data,
    set_todo_deadline,
    update_todo_text,
    save_todo_data_with_groups,
    load_todo_data_with_groups,
    save_group_data,
    load_group_data,
    save_app_settings,
    load_app_settings,
    apply_opacity,
    save_window_position,
    load_window_position,
};

// 创建一个全局变量来跟踪Win+D状态
static WIN_D_PRESSED: AtomicBool = AtomicBool::new(false);

// Tauri 命令：获取应用版本
#[tauri::command]
async fn get_app_version(app: tauri::AppHandle) -> Result<String, String> {
    Ok(app.package_info().version.to_string())
}

// Tauri 命令：检查是否为开发模式
#[tauri::command]
async fn is_dev_mode() -> Result<bool, String> {
    Ok(cfg!(debug_assertions))
}

// Tauri 命令：退出应用
#[tauri::command]
async fn quit_app(app: tauri::AppHandle) -> Result<(), String> {
    // 关闭所有窗口
    if let Some(main_window) = app.get_webview_window("main") {
        let _ = main_window.close();
    }
    if let Some(settings_window) = app.get_webview_window("settings") {
        let _ = settings_window.close();
    }
    
    // 退出应用
    app.exit(0);
    Ok(())
}

// Tauri 命令：发送主题更改事件
#[tauri::command]
async fn emit_theme_changed(app: tauri::AppHandle, theme: String) -> Result<(), String> {
    app.emit("theme-changed", theme).map_err(|e| e.to_string())?;
    Ok(())
}

// Tauri 命令：发送高优先级颜色更改事件
#[tauri::command]
async fn emit_priority_color_changed(app: tauri::AppHandle, color: String) -> Result<(), String> {
    app.emit("priority-color-changed", color).map_err(|e| e.to_string())?;
    Ok(())
}

// Tauri 命令：测试通知功能
#[tauri::command]
async fn test_notification(app: tauri::AppHandle) -> Result<(), String> {
    use tauri_plugin_notification::NotificationExt;
    
    app.notification()
        .builder()
        .title("DeskHive 测试通知")
        .body("通知功能正常工作！")
        .show()
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

// 检查是否已经有一个实例在运行
#[cfg(target_os = "windows")]
fn is_single_instance() -> bool {
    unsafe {
        let app_name = "DeskHive_Single_Instance_Mutex";
        let wide_name: Vec<u16> = OsStr::new(app_name).encode_wide().chain(Some(0)).collect();
        
        let mutex = CreateMutexW(
            None,
            true,
            windows::core::PCWSTR(wide_name.as_ptr()),
        );
        
        match mutex {
            Ok(handle) => {
                let last_error = windows::Win32::Foundation::GetLastError();
                if last_error.0 == 183 { // ERROR_ALREADY_EXISTS
                    let _ = CloseHandle(handle);
                    return false; // 已经存在实例
                }
                true // 没有其他实例在运行
            }
            Err(_) => {
                false // 创建互斥锁失败
            }
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 检查是否已经有一个实例在运行
    #[cfg(target_os = "windows")]
    {
        if !is_single_instance() {
            // 已经有一个实例在运行，直接退出
            return;
        }
    }
    
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![
            // 数据相关命令
            save_todo_data, 
            load_todo_data,
            set_todo_deadline,
            update_todo_text,
            save_todo_data_with_groups,
            load_todo_data_with_groups,
            save_group_data,
            load_group_data,
            save_app_settings,
            load_app_settings,
            apply_opacity,
            save_window_position,
            load_window_position,
            
            // 窗口管理命令
            window::management::toggle_main_window,
            window::management::show_main_window,
            window::management::minimize_to_tray,
            window::management::restore_from_tray,
            window::management::open_settings_window,
            window::management::close_settings_window,
            window::management::reset_window_position,
            
            // 系统相关命令
            system::date_info::get_current_date,
            get_app_version,
            is_dev_mode,
            quit_app,
            emit_theme_changed,
            emit_priority_color_changed,
            test_notification
        ])
        .setup(|app| {
            // 初始化日志系统
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // 创建系统托盘
            system::tray::create_tray(app)?;

            // 启动通知检查定时器
            let app_handle = app.handle().clone();
            std::thread::spawn(move || {
                let notification_manager = notification::NotificationManager::new(app_handle);
                loop {
                    // 每分钟检查一次
                    std::thread::sleep(std::time::Duration::from_secs(60));
                    notification_manager.check_and_notify();
                }
            });

            // 获取主窗口
            if let Some(window) = app.get_webview_window("main") {
                // 同步加载并应用保存的设置和位置（在显示窗口之前）
                let app_handle = app.handle().clone();
                let window_clone = window.clone();
                
                // 先加载应用设置，用于后续应用透明度
                let loaded_settings = tauri::async_runtime::block_on(async {
                    load_app_settings(app_handle.clone()).await.ok()
                });
                
                // 使用阻塞调用确保在显示窗口前完成位置设置
                tauri::async_runtime::block_on(async {
                    // 默认总是加载和应用保存的位置
                    match load_window_position(app_handle.clone()).await {
                        Ok(Some(position)) => {
                            println!("加载保存的位置: ({}, {})", position.x, position.y);
                            // 如果有保存的位置，验证并修正位置确保窗口完全可见
                            let (fixed_x, fixed_y) = window::position::validate_and_fix_position(position.x, position.y);
                            
                            if let Err(e) = window_clone.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                                x: fixed_x,
                                y: fixed_y,
                            })) {
                                println!("设置窗口位置失败: {}", e);
                            } else {
                                println!("成功设置窗口位置: ({}, {})", fixed_x, fixed_y);
                            }
                            
                            // 如果位置被修正了，保存新的位置
                            if fixed_x != position.x || fixed_y != position.y {
                                let _ = save_window_position(app_handle.clone(), fixed_x, fixed_y).await;
                            }
                        }
                        Ok(None) => {
                            println!("首次启动，设置到右上角");
                            // 如果是第一次打开，设置到右上角
                            let (x, y) = window::position::get_top_right_position();
                            
                            if let Err(e) = window_clone.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                                x,
                                y,
                            })) {
                                println!("设置右上角位置失败: {}", e);
                            } else {
                                println!("成功设置右上角位置: ({}, {})", x, y);
                            }
                            
                            // 保存初始位置
                            let _ = save_window_position(app_handle.clone(), x, y).await;
                        }
                        Err(e) => {
                            println!("加载位置失败: {}, 使用右上角位置", e);
                            // 加载失败，使用右上角位置
                            let (x, y) = window::position::get_top_right_position();
                            let _ = window_clone.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                                x,
                                y,
                            }));
                        }
                    }
                });
                
                // 确保位置设置完成后再显示窗口
                let _ = window.show();
                
                // 根据设置决定是否获取焦点（静默启动）
                if let Some(ref settings) = loaded_settings {
                    if !settings.silent_start {
                        let _ = window.set_focus();
                    }
                } else {
                    let _ = window.set_focus();
                }
                
                // 延迟应用透明度和窗口层级设置，确保窗口完全初始化
                if let Some(settings) = loaded_settings {
                    let window_for_settings = window.clone();
                    let opacity_value = settings.opacity;
                    let window_level = settings.window_level.clone();
                    std::thread::spawn(move || {
                        std::thread::sleep(std::time::Duration::from_millis(100));
                        
                        // 应用透明度
                        if let Err(e) = window::opacity::set_window_opacity(&window_for_settings, opacity_value) {
                            println!("应用启动透明度失败: {}", e);
                        } else {
                            println!("成功应用启动透明度: {}", opacity_value);
                        }
                        
                        // 应用窗口层级
                        match window_level.as_str() {
                            "always_on_top" => {
                                let _ = window_for_settings.set_always_on_top(true);
                                println!("应用窗口层级: 置于顶层");
                            },
                            "always_on_bottom" => {
                                #[cfg(target_os = "windows")]
                                {
                                    use windows::Win32::Foundation::HWND;
                                    use windows::Win32::UI::WindowsAndMessaging::{
                                        SetWindowPos, HWND_BOTTOM, SWP_NOMOVE, SWP_NOSIZE, SWP_NOACTIVATE
                                    };
                                    
                                    if let Ok(hwnd) = window_for_settings.hwnd() {
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
                                let _ = window_for_settings.set_always_on_top(false);
                                println!("应用窗口层级: 置于桌面");
                            },
                            _ => {
                                let _ = window_for_settings.set_always_on_top(false);
                                println!("应用窗口层级: 普通");
                            }
                        }
                    });
                }

                // 在Tauri 2.x中处理窗口事件
                #[cfg(target_os = "windows")]
                {
                    let win_handle = window.clone();
                    
                    // 监听窗口事件
                    let app_handle_for_events = app.handle().clone();
                    
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
                            // 检测窗口移动事件 - 验证位置并保存
                            tauri::WindowEvent::Moved(position) => {
                                let app_handle = app_handle_for_events.clone();
                                let window = win_handle.clone();
                                let x = position.x;
                                let y = position.y;
                                
                                // 验证并修正位置
                                let (fixed_x, fixed_y) = window::position::validate_and_fix_position(x, y);
                                
                                // 如果位置被修正了，更新窗口位置
                                if fixed_x != x || fixed_y != y {
                                    let _ = window.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                                        x: fixed_x,
                                        y: fixed_y,
                                    }));
                                }
                                
                                // 异步保存位置
                                tauri::async_runtime::spawn(async move {
                                    let _ = save_window_position(app_handle, fixed_x, fixed_y).await;
                                });
                            }
                            // 检测窗口关闭请求事件 - 最小化到托盘而不是退出
                            tauri::WindowEvent::CloseRequested { api, .. } => {
                                // 阻止默认的关闭行为
                                api.prevent_close();
                                
                                // 在隐藏前保存当前位置
                                if let Ok(current_position) = win_handle.outer_position() {
                                    let app_handle = app_handle_for_events.clone();
                                    tauri::async_runtime::spawn(async move {
                                        let _ = save_window_position(app_handle, current_position.x, current_position.y).await;
                                    });
                                }
                                
                                // 隐藏窗口到托盘
                                let _ = win_handle.hide();
                            }
                            // 其他事件
                            _ => {}
                        }
                    });

                    // 定时器：只处理 Win+D 恢复
                    let win_handle2 = window.clone();
                    std::thread::spawn(move || {
                        loop {
                            std::thread::sleep(std::time::Duration::from_millis(300));

                            // 检查窗口是否还存在
                            if win_handle2.is_visible().is_err() {
                                // 窗口可能已关闭，退出循环
                                break;
                            }

                            // 检查Win+D状态
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
                        }
                    });
                }
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}