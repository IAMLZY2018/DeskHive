#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{SetWindowLongPtrW, GetWindowLongPtrW, SetLayeredWindowAttributes, GWL_EXSTYLE, WS_EX_LAYERED, LWA_ALPHA};
#[cfg(target_os = "windows")]
use windows::Win32::Foundation::{HWND, COLORREF};
#[cfg(target_os = "windows")]
use raw_window_handle::{HasWindowHandle, RawWindowHandle};

// 设置窗口透明度的辅助函数
#[cfg(target_os = "windows")]
pub fn set_window_opacity(window: &tauri::WebviewWindow, opacity: f64) -> Result<(), String> {
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
pub fn set_window_opacity(_window: &tauri::WebviewWindow, _opacity: f64) -> Result<(), String> {
    // 在非Windows平台上，暂时不支持透明度设置
    Ok(())
}