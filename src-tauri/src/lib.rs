use tauri::Manager;
use std::sync::atomic::{AtomicBool, Ordering};

#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{WM_SYSCOMMAND, SC_MINIMIZE, WM_SHOWWINDOW};
#[cfg(target_os = "windows")]
use windows::Win32::Foundation::{HWND, WPARAM, LPARAM, LRESULT, BOOL};

// 创建一个全局变量来跟踪Win+D状态
static WIN_D_PRESSED: AtomicBool = AtomicBool::new(false);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
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
        // 使用alwaysOnTop而不是alwaysOnBottom
        // window.set_always_on_bottom(true).unwrap();
        
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
                      // 尝试恢复窗口
                      let _ = win.show();
                      let _ = win.set_focus();
                      WIN_D_PRESSED.store(false, Ordering::SeqCst);
                    }
                  });
                }
              },
              // 检测窗口隐藏事件
              tauri::WindowEvent::CloseRequested { .. } => {
                // 不做任何处理，让窗口正常关闭
              },
              // 其他事件
              _ => {}
            }
          });
          
          // 添加定时器，但只在检测到Win+D时才尝试恢复窗口
          let win_handle2 = window.clone();
          std::thread::spawn(move || {
            loop {
              std::thread::sleep(std::time::Duration::from_millis(500));
              
              // 只有当WIN_D_PRESSED为true时才检查窗口可见性
              if WIN_D_PRESSED.load(Ordering::SeqCst) {
                // 检查窗口是否可见
                match win_handle2.is_visible() {
                  Ok(visible) => {
                    if !visible {
                      // 窗口不可见，尝试恢复
                      let _ = win_handle2.show();
                      let _ = win_handle2.set_focus();
                      // 恢复后重置标志
                      WIN_D_PRESSED.store(false, Ordering::SeqCst);
                    } else {
                      // 窗口已经可见，重置标志
                      WIN_D_PRESSED.store(false, Ordering::SeqCst);
                    }
                  },
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
