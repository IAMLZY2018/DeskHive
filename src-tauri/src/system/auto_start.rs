#[cfg(target_os = "windows")]
use windows::Win32::System::Registry::{HKEY_CURRENT_USER, RegSetValueExW, RegDeleteValueW, RegOpenKeyExW, RegCloseKey, KEY_WRITE, REG_SZ};

// Windows平台下的开机自启动实现
#[cfg(target_os = "windows")]
pub fn set_auto_start(app_handle: &tauri::AppHandle, enable: bool) -> Result<(), String> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    
    unsafe {
        // 打开注册表键
        let subkey = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run";
        let subkey_wide: Vec<u16> = OsStr::new(subkey).encode_wide().chain(Some(0)).collect();
        
        let mut hkey = windows::Win32::System::Registry::HKEY::default();
        let result = RegOpenKeyExW(
            HKEY_CURRENT_USER,
            windows::core::PCWSTR(subkey_wide.as_ptr()),
            0,
            KEY_WRITE,
            &mut hkey,
        );
        
        if result.is_err() {
            return Err(format!("无法打开注册表键: {:?}", result));
        }
        
        // 获取应用程序路径
        let app_name = app_handle.package_info().name.clone();
        let app_path = std::env::current_exe()
            .map_err(|e| format!("无法获取应用程序路径: {}", e))?;
        
        if enable {
            // 将应用程序路径写入注册表
            let path_str = app_path.to_string_lossy().to_string();
            let path_wide: Vec<u16> = OsStr::new(&path_str).encode_wide().chain(Some(0)).collect();
            let app_name_wide: Vec<u16> = OsStr::new(&app_name).encode_wide().chain(Some(0)).collect();
            
            let path_bytes = path_wide.as_ptr() as *const u8;
            let path_bytes_len = (path_wide.len() * 2) as u32;
            
            let result = RegSetValueExW(
                hkey,
                windows::core::PCWSTR(app_name_wide.as_ptr()),
                0,
                REG_SZ,
                Some(std::slice::from_raw_parts(path_bytes, path_bytes_len as usize)),
            );
            
            if result.is_err() {
                RegCloseKey(hkey);
                return Err(format!("无法设置注册表值: {:?}", result));
            }
        } else {
            // 从注册表中删除应用程序路径
            let app_name_wide: Vec<u16> = OsStr::new(&app_name).encode_wide().chain(Some(0)).collect();
            
            let result = RegDeleteValueW(
                hkey,
                windows::core::PCWSTR(app_name_wide.as_ptr()),
            );
            
            if result.is_err() && !result.to_hresult().is_ok() {
                // 忽略"找不到"错误，因为这可能是首次禁用
                RegCloseKey(hkey);
                return Err(format!("无法删除注册表值: {:?}", result));
            }
        }
        
        RegCloseKey(hkey);
    }
    
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn set_auto_start(_app_handle: &tauri::AppHandle, _enable: bool) -> Result<(), String> {
    // 在非Windows平台上，暂时不支持开机自启动
    Err("当前平台不支持开机自启动功能".to_string())
}