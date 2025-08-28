use tauri::{menu::{MenuBuilder, MenuItemBuilder}, tray::{TrayIconBuilder, TrayIconEvent}};

use crate::window::management::{toggle_main_window, open_settings_window, show_main_window};
use crate:: quit_app;

// 创建系统托盘菜单和事件处理
pub fn create_tray(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    // 创建系统托盘菜单
    let show_hide = MenuItemBuilder::with_id("show_hide", "显示/隐藏").build(app)?;
    let settings = MenuItemBuilder::with_id("settings", "设置").build(app)?;
    let quit = MenuItemBuilder::with_id("quit", "退出").build(app)?;
    
    let menu = MenuBuilder::new(app)
        .items(&[&show_hide, &settings, &quit])
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