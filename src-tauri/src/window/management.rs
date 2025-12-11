use tauri::{Manager};

use crate::data::load_app_settings;

// Tauri 命令：显示/隐藏主窗口
#[tauri::command]
pub async fn toggle_main_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        match window.is_visible() {
            Ok(true) => {
                let _ = window.hide();
            }
            Ok(false) => {
                let _ = window.show();
                // 始终获取焦点
                let settings = load_app_settings(app.clone()).await.unwrap_or_default();
                let _ = window.set_focus();
                
                // 如果窗口层级是"置于桌面"，需要重新应用该设置
                // 因为 show() 和 set_focus() 可能会改变窗口层级
                if settings.window_level == "always_on_bottom" {
                    #[cfg(target_os = "windows")]
                    {
                        use windows::Win32::Foundation::HWND;
                        use windows::Win32::UI::WindowsAndMessaging::{
                            SetWindowPos, HWND_BOTTOM, SWP_NOMOVE, SWP_NOSIZE, SWP_NOACTIVATE
                        };
                        
                        if let Ok(hwnd) = window.hwnd() {
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
                }
            }
            Err(_) => {
                let _ = window.show();
                // 始终获取焦点
                let settings = load_app_settings(app.clone()).await.unwrap_or_default();
                let _ = window.set_focus();
                
                // 如果窗口层级是"置于桌面"，需要重新应用该设置
                if settings.window_level == "always_on_bottom" {
                    #[cfg(target_os = "windows")]
                    {
                        use windows::Win32::Foundation::HWND;
                        use windows::Win32::UI::WindowsAndMessaging::{
                            SetWindowPos, HWND_BOTTOM, SWP_NOMOVE, SWP_NOSIZE, SWP_NOACTIVATE
                        };
                        
                        if let Ok(hwnd) = window.hwnd() {
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
                }
            }
        }
    }
    Ok(())
}

// Tauri 命令：只显示主窗口（不隐藏）
#[tauri::command]
pub async fn show_main_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        
        // 加载设置
        let settings = load_app_settings(app.clone()).await.unwrap_or_default();
        
        // 使用 Windows API 强制获取焦点
        #[cfg(target_os = "windows")]
        {
            use windows::Win32::Foundation::HWND;
            use windows::Win32::UI::WindowsAndMessaging::{
                SetForegroundWindow, SetWindowPos, HWND_BOTTOM, SWP_NOMOVE, SWP_NOSIZE,
                ShowWindow, SW_SHOW
            };
            
            if let Ok(hwnd) = window.hwnd() {
                unsafe {
                    let window_hwnd = HWND(hwnd.0 as _);
                    
                    // 确保窗口可见
                    ShowWindow(window_hwnd, SW_SHOW);
                    
                    // 强制设置为前台窗口（获取焦点）
                    SetForegroundWindow(window_hwnd);
                    
                    // 如果窗口层级是"置于桌面"，在获取焦点后重新应用该设置
                    if settings.window_level == "always_on_bottom" {
                        // 不使用 SWP_NOACTIVATE，保持窗口激活状态
                        let _ = SetWindowPos(
                            window_hwnd,
                            HWND_BOTTOM,
                            0, 0, 0, 0,
                            SWP_NOMOVE | SWP_NOSIZE
                        );
                    }
                }
            }
        }
        
        // 非 Windows 平台使用 Tauri 的 set_focus
        #[cfg(not(target_os = "windows"))]
        {
            let _ = window.set_focus();
        }
    }
    Ok(())
}

// Tauri 命令：最小化到托盘
#[tauri::command]
pub async fn minimize_to_tray(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }
    Ok(())
}

// Tauri 命令：从托盘恢复
#[tauri::command]
pub async fn restore_from_tray(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        // 始终获取焦点
        let settings = load_app_settings(app.clone()).await.unwrap_or_default();
        let _ = window.set_focus();
        
        // 如果窗口层级是"置于桌面"，需要重新应用该设置
        if settings.window_level == "always_on_bottom" {
            #[cfg(target_os = "windows")]
            {
                use windows::Win32::Foundation::HWND;
                use windows::Win32::UI::WindowsAndMessaging::{
                    SetWindowPos, HWND_BOTTOM, SWP_NOMOVE, SWP_NOSIZE, SWP_NOACTIVATE
                };
                
                if let Ok(hwnd) = window.hwnd() {
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
        }
    }
    Ok(())
}

// Tauri 命令：关闭设置窗口
#[tauri::command]
pub async fn close_settings_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("settings") {
        window.close().map_err(|e| format!("关闭设置窗口失败: {}", e))?;
    }
    Ok(())
}

// Tauri 命令：打开设置窗口
#[tauri::command]
pub async fn open_settings_window(app: tauri::AppHandle) -> Result<(), String> {
    // 检查设置窗口是否已经存在
    if let Some(window) = app.get_webview_window("settings") {
        println!("设置窗口已存在，尝试显示并获取焦点");
        
        // 如果窗口已存在，使用 Windows API 强制获取焦点
        #[cfg(target_os = "windows")]
        {
            use windows::Win32::Foundation::HWND;
            use windows::Win32::UI::WindowsAndMessaging::{
                SetForegroundWindow, ShowWindow, SW_RESTORE, SW_SHOW, IsIconic
            };
            
            if let Ok(hwnd) = window.hwnd() {
                unsafe {
                    let window_hwnd = HWND(hwnd.0 as _);
                    
                    // 检查窗口是否最小化
                    if IsIconic(window_hwnd).as_bool() {
                        println!("窗口已最小化，恢复窗口");
                        ShowWindow(window_hwnd, SW_RESTORE);
                    } else {
                        println!("窗口未最小化，显示窗口");
                        ShowWindow(window_hwnd, SW_SHOW);
                    }
                    
                    // 强制设置为前台窗口（获取焦点）
                    let result = SetForegroundWindow(window_hwnd);
                    println!("SetForegroundWindow 结果: {:?}", result);
                }
            }
        }
        
        // 非 Windows 平台使用 Tauri 的方法
        #[cfg(not(target_os = "windows"))]
        {
            let _ = window.show();
            let _ = window.set_focus();
        }
        
        // 同时使用 Tauri 的方法作为备用
        let _ = window.show();
        let _ = window.set_focus();
        let _ = window.unminimize();
        
        println!("设置窗口显示完成");
        return Ok(());
    }

    println!("创建新的设置窗口");
    
    // 使用 Tauri 2.x 的 API 创建新窗口
    let _settings_window = tauri::WebviewWindowBuilder::new(
        &app,
        "settings",
        tauri::WebviewUrl::App("settings".into()),
    )
    .title("设置")
    .inner_size(800.0, 600.0)
    .min_inner_size(800.0, 600.0)
    .center()
    .resizable(false)
    .decorations(false)
    .always_on_top(false)
    .skip_taskbar(false)
    .icon(app.default_window_icon().unwrap().clone())
    .map_err(|e| format!("设置窗口图标失败: {}", e))?
    .build()
    .map_err(|e| format!("创建设置窗口失败: {}", e))?;

    println!("新设置窗口创建完成");

    // 注意：设置窗口保持不透明，不应用透明度设置
    // 透明度设置只应用于主窗口

    Ok(())
}

// Tauri 命令：重置窗口位置到屏幕中心
#[tauri::command]
pub async fn reset_window_position(app: tauri::AppHandle) -> Result<(), String> {
    use crate::window::position;
    use crate::data::{save_window_position, save_app_settings};
    
    if let Some(window) = app.get_webview_window("main") {
        // 获取屏幕中心位置
        let (x, y) = position::get_center_position();
        
        // 设置窗口位置
        window.set_position(tauri::Position::Physical(tauri::PhysicalPosition { x, y }))
            .map_err(|e| format!("设置窗口位置失败: {}", e))?;
        
        // 保存新位置
        save_window_position(app.clone(), x, y).await?;
        
        // 加载当前设置并关闭"禁止拖动窗口"
        if let Ok(mut settings) = load_app_settings(app.clone()).await {
            settings.disable_drag = false;
            // 保存更新后的设置
            save_app_settings(app.clone(), settings).await?;
            println!("已关闭禁止拖动窗口设置");
        }
        
        // 显示并聚焦窗口
        let _ = window.show();
        let _ = window.set_focus();
        
        println!("窗口位置已重置到屏幕中心: ({}, {})", x, y);
    }
    
    Ok(())
}

