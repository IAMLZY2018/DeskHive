use tauri::{menu::{MenuBuilder, MenuItemBuilder}, tray::{TrayIconBuilder, TrayIconEvent}, Manager};

use crate::window::management::{toggle_main_window, open_settings_window, show_main_window};
use crate::quit_app;

// 创建系统托盘菜单和事件处理
pub fn create_tray(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    // 创建系统托盘菜单
    let show_hide = MenuItemBuilder::with_id("show_hide", "显示/隐藏").build(app)?;
    let always_on_top = MenuItemBuilder::with_id("always_on_top", "置于顶层").build(app)?;
    let pin_to_desktop = MenuItemBuilder::with_id("pin_to_desktop", "置于桌面").build(app)?;
    let settings = MenuItemBuilder::with_id("settings", "设置").build(app)?;
    let quit = MenuItemBuilder::with_id("quit", "退出").build(app)?;
    
    let menu = MenuBuilder::new(app)
        .items(&[&show_hide, &always_on_top, &pin_to_desktop, &settings, &quit])
        .build()?;

    // 创建系统托盘图标
    let _tray = TrayIconBuilder::with_id("main")
        .menu(&menu)
        .tooltip("Todo 桌面助手")
        .icon(app.default_window_icon().unwrap().clone())
        .on_tray_icon_event(|tray, event| {
            match event {
                TrayIconEvent::Click { button: tauri::tray::MouseButton::Left, .. } => {
                    // 左键单击只显示窗口
                    let app = tray.app_handle().clone();
                    tauri::async_runtime::spawn(async move {
                        let _ = show_main_window(app).await;
                    });
                }
                _ => {}
            }
        })
        .on_menu_event(|app, event| {
            match event.id().as_ref() {
                "show_hide" => {
                    let app_handle = app.clone();
                    tauri::async_runtime::spawn(async move {
                        let _ = toggle_main_window(app_handle).await;
                    });
                }
                "always_on_top" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let is_always_on_top = window.is_always_on_top().unwrap_or(false);
                        let _ = window.set_always_on_top(!is_always_on_top);
                    }
                }
                "pin_to_desktop" => {
                    #[cfg(target_os = "windows")]
                    {
                        use windows::Win32::Foundation::HWND;
                        use windows::Win32::UI::WindowsAndMessaging::{
                            SetParent, GetParent,
                            SetWindowPos, HWND_BOTTOM, SWP_NOMOVE, SWP_NOSIZE, SWP_NOACTIVATE
                        };
                        
                        if let Some(window) = app.get_webview_window("main") {
                            if let Ok(hwnd) = window.hwnd() {
                                unsafe {
                                    let window_hwnd = HWND(hwnd.0 as _);
                                    let current_parent = GetParent(window_hwnd);
                                    
                                    // 检查当前是否已经固定到桌面
                                    if current_parent.0 != 0 {
                                        // 如果已固定，则恢复为普通窗口
                                        let _ = SetParent(window_hwnd, HWND(0));
                                        // 确保窗口可见
                                        let _ = window.show();
                                    } else {
                                        // 如果未固定，则固定到桌面
                                        // 使用 SetWindowPos 将窗口置于底层，而不是改变父窗口
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
                "settings" => {
                    let app_handle = app.clone();
                    tauri::async_runtime::spawn(async move {
                        let _ = open_settings_window(app_handle).await;
                    });
                }
                "quit" => {
                    let app_handle = app.clone();
                    tauri::async_runtime::spawn(async move {
                        let _ = quit_app(app_handle).await;
                    });
                }
                _ => {}
            }
        })
        .build(app)?;

    Ok(())
}