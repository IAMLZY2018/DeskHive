use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use std::fs;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use tauri::{Manager, Emitter, tray::{TrayIconBuilder, TrayIconEvent}, menu::{MenuBuilder, MenuItemBuilder}};

#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{SetWindowLongPtrW, GetWindowLongPtrW, SetLayeredWindowAttributes, GWL_EXSTYLE, WS_EX_LAYERED, LWA_ALPHA};
#[cfg(target_os = "windows")]
use windows::Win32::Foundation::{HWND, COLORREF};
#[cfg(target_os = "windows")]
use windows::Win32::System::Registry::{HKEY_CURRENT_USER, RegSetValueExW, RegDeleteValueW, RegOpenKeyExW, RegCloseKey, KEY_WRITE, REG_SZ};
#[cfg(target_os = "windows")]
use raw_window_handle::{HasWindowHandle, RawWindowHandle};

// 创建一个全局变量来跟踪Win+D状态
static WIN_D_PRESSED: AtomicBool = AtomicBool::new(false);

// 拖动防抖结构
#[derive(Clone)]
struct DragDebouncer {
    last_move_time: Arc<Mutex<Option<Instant>>>,
    pending_position: Arc<Mutex<Option<(i32, i32)>>>,
}

// Todo数据结构
#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    text: String,
    completed: bool,
    created_at: i64, // Unix时间戳（秒）
    #[serde(default)] // 为了兼容旧数据，设为默认值
    deadline: Option<i64>, // 截止时间，Unix时间戳（秒），可选
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
    disable_drag: bool,
    #[serde(default = "default_auto_show")]
    auto_show: bool,
    #[serde(default = "default_minimize_to_tray")]
    minimize_to_tray: bool,
    #[serde(default = "default_auto_start")]
    auto_start: bool,
    #[serde(default = "default_silent_start")]
    silent_start: bool,
}

// 默认值函数
fn default_auto_show() -> bool {
    true
}

fn default_minimize_to_tray() -> bool {
    true
}

fn default_auto_start() -> bool {
    false
}

fn default_silent_start() -> bool {
    false
}

// Windows平台下的开机自启动实现
#[cfg(target_os = "windows")]
fn set_auto_start(app_handle: &tauri::AppHandle, enable: bool) -> Result<(), String> {
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
fn set_auto_start(_app_handle: &tauri::AppHandle, _enable: bool) -> Result<(), String> {
    // 在非Windows平台上，暂时不支持开机自启动
    Err("当前平台不支持开机自启动功能".to_string())
}

// 窗口位置结构
#[derive(Serialize, Deserialize, Clone)]
struct WindowPosition {
    x: i32,
    y: i32,
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
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
            
        return Ok(TodoData {
            pending_todos: vec![
                Todo { 
                    text: "欢迎使用DeskHive桌面助手".to_string(),
                    completed: false,
                    created_at: now - 3600, // 1小时前创建
                    deadline: None, // 无截止时间
                },
                Todo { 
                    text: "测试天数指示器功能 - 3天前创建".to_string(),
                    completed: false,
                    created_at: now - (3 * 24 * 3600), // 3天前创建
                    deadline: Some(now + (2 * 24 * 3600)), // 设置2天后截止，用于测试倒计时
                },
                Todo { 
                    text: "测试天数指示器功能 - 7天前创建".to_string(),
                    completed: false,
                    created_at: now - (7 * 24 * 3600), // 7天前创建
                    deadline: None, // 无截止时间
                },
            ],
            completed_todos: vec![
                Todo { 
                    text: "已完成的会收纳到这里，可以双击删除哦~".to_string(),
                    completed: true,
                    created_at: now - (5 * 24 * 3600), // 5天前创建
                    deadline: Some(now - (1 * 24 * 3600)), // 设置1天前截止（已过期），用于测试过期显示
                },
            ],
        });
    }
    
    let json_data = fs::read_to_string(&file_path)
        .map_err(|e| format!("读取文件失败: {}", e))?;
    
    let todo_data: TodoData = serde_json::from_str(&json_data)
        .map_err(|e| format!("解析JSON失败: {}", e))?;
    
    Ok(todo_data)
}

// Tauri 命令：设置todo截止时间
#[tauri::command]
async fn set_todo_deadline(
    app: tauri::AppHandle, 
    todo_text: String, 
    is_completed: bool,
    deadline: Option<i64>
) -> Result<(), String> {
    // 先加载当前数据
    let mut todo_data = load_todo_data(app.clone()).await?;
    
    // 查找并更新对应的todo项
    let found = if is_completed {
        // 在已完成列表中查找
        todo_data.completed_todos.iter_mut()
            .find(|todo| todo.text == todo_text)
    } else {
        // 在待完成列表中查找
        todo_data.pending_todos.iter_mut()
            .find(|todo| todo.text == todo_text)
    };
    
    if let Some(todo) = found {
        todo.deadline = deadline;
        // 保存更新后的数据
        save_todo_data(app, todo_data.pending_todos, todo_data.completed_todos).await?;
        Ok(())
    } else {
        Err("未找到指定的todo项".to_string())
    }
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
    
    // 处理开机自启动设置
    if let Ok(old_settings) = load_app_settings(app.clone()).await {
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
        
        // 不再设置置顶状态，always_on_top 现在表示"记住窗口位置"
        // let _ = main_window.set_always_on_top(settings.always_on_top);
        
        // 通知前端更新拖动设置
        let _ = main_window.emit("drag-setting-changed", settings.disable_drag);
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
            disable_drag: false,
            auto_show: true,
            minimize_to_tray: true,
            auto_start: false,
            silent_start: false,
        });
    }
    
    let json_data = fs::read_to_string(&file_path)
        .map_err(|e| format!("读取设置文件失败: {}", e))?;
    
    let settings: AppSettings = serde_json::from_str(&json_data)
        .map_err(|e| format!("解析设置JSON失败: {}", e))?;
    
    Ok(settings)
}

// Tauri 命令：应用透明度设置（只应用于主窗口）
#[tauri::command]
async fn apply_opacity(app: tauri::AppHandle, opacity: f64) -> Result<(), String> {
    // 只对主窗口应用透明度，设置窗口保持不透明
    if let Some(main_window) = app.get_webview_window("main") {
        set_window_opacity(&main_window, opacity)?;
    }
    Ok(())
}

// Tauri 命令：保存窗口位置
#[tauri::command]
async fn save_window_position(app: tauri::AppHandle, x: i32, y: i32) -> Result<(), String> {
    let data_dir = get_data_dir(&app)?;
    let file_path = data_dir.join("window_position.json");
    
    let position = WindowPosition { x, y };
    
    let json_data = serde_json::to_string_pretty(&position)
        .map_err(|e| format!("序列化窗口位置失败: {}", e))?;
    
    fs::write(&file_path, json_data)
        .map_err(|e| format!("写入窗口位置文件失败: {}", e))?;
    
    Ok(())
}

// Tauri 命令：加载窗口位置
#[tauri::command]
async fn load_window_position(app: tauri::AppHandle) -> Result<Option<WindowPosition>, String> {
    let data_dir = get_data_dir(&app)?;
    let file_path = data_dir.join("window_position.json");
    
    if !file_path.exists() {
        return Ok(None);
    }
    
    let json_data = fs::read_to_string(&file_path)
        .map_err(|e| format!("读取窗口位置文件失败: {}", e))?;
    
    let position: WindowPosition = serde_json::from_str(&json_data)
        .map_err(|e| format!("解析窗口位置JSON失败: {}", e))?;
    
    Ok(Some(position))
}

// 获取屏幕工作区信息
#[cfg(target_os = "windows")]
fn get_screen_work_area() -> (i32, i32, i32, i32) {
    use windows::Win32::UI::WindowsAndMessaging::{SystemParametersInfoW, SPI_GETWORKAREA};
    use windows::Win32::Foundation::RECT;
    
    unsafe {
        let mut work_area = RECT::default();
        let result = SystemParametersInfoW(
            SPI_GETWORKAREA,
            0,
            Some(&mut work_area as *mut _ as *mut _),
            windows::Win32::UI::WindowsAndMessaging::SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        );
        
        if result.as_bool() {
            println!("工作区: left={}, top={}, right={}, bottom={}", 
                work_area.left, work_area.top, work_area.right, work_area.bottom);
            (work_area.left, work_area.top, work_area.right, work_area.bottom)
        } else {
            // 如果获取工作区失败，使用屏幕尺寸
            use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
            let screen_width = GetSystemMetrics(SM_CXSCREEN);
            let screen_height = GetSystemMetrics(SM_CYSCREEN);
            println!("使用屏幕尺寸: {}x{}", screen_width, screen_height);
            (0, 0, screen_width, screen_height)
        }
    }
}

// 获取屏幕尺寸并计算右上角位置
#[cfg(target_os = "windows")]
fn get_top_right_position() -> (i32, i32) {
    let (work_left, work_top, work_right, work_bottom) = get_screen_work_area();
    
    // 窗口尺寸 (从配置中获取)
    let window_width = 300;
    let window_height = 420;
    
    // 工作区的实际宽度和高度
    let work_width = work_right - work_left;
    let work_height = work_bottom - work_top;
    
    // 计算右上角位置，确保窗口完全在工作区内
    let x = work_left + work_width - window_width - 10; // 右边留10像素边距
    let y = work_top + 50; // 上边留50像素边距
    
    // 确保位置不会超出工作区边界
    let final_x = x.max(work_left).min(work_right - window_width);
    let final_y = y.max(work_top).min(work_bottom - window_height);
    
    println!("工作区尺寸: {}x{}, 窗口尺寸: {}x{}, 计算位置: ({}, {})", 
        work_width, work_height, window_width, window_height, final_x, final_y);
    
    (final_x, final_y)
}

#[cfg(not(target_os = "windows"))]
fn get_top_right_position() -> (i32, i32) {
    // 在非Windows平台上，使用默认位置
    (1000, 50)
}

// 严格检查并修正窗口位置，确保窗口完全在工作区内
#[cfg(target_os = "windows")]
fn validate_and_fix_position(x: i32, y: i32) -> (i32, i32) {
    let (work_left, work_top, work_right, work_bottom) = get_screen_work_area();
    
    // 窗口尺寸
    let window_width = 300;
    let window_height = 420;
    
    // 严格的边界检查，确保窗口完全在工作区内
    let fixed_x = x.max(work_left).min(work_right - window_width);
    let fixed_y = y.max(work_top).min(work_bottom - window_height);
    
    // 额外检查：确保窗口不会部分超出边界
    let final_x = if fixed_x + window_width > work_right {
        work_right - window_width
    } else if fixed_x < work_left {
        work_left
    } else {
        fixed_x
    };
    
    let final_y = if fixed_y + window_height > work_bottom {
        work_bottom - window_height
    } else if fixed_y < work_top {
        work_top
    } else {
        fixed_y
    };
    
    if final_x != x || final_y != y {
        println!("位置修正: ({}, {}) -> ({}, {})", x, y, final_x, final_y);
        println!("窗口边界: 左={}, 上={}, 右={}, 下={}", 
            final_x, final_y, final_x + window_width, final_y + window_height);
        println!("工作区边界: 左={}, 上={}, 右={}, 下={}", 
            work_left, work_top, work_right, work_bottom);
    }
    
    (final_x, final_y)
}

#[cfg(not(target_os = "windows"))]
fn validate_and_fix_position(x: i32, y: i32) -> (i32, i32) {
    // 在非Windows平台上，使用默认屏幕尺寸进行检查
    let screen_width = 1920;  // 假设默认屏幕宽度
    let screen_height = 1080; // 假设默认屏幕高度
    let window_width = 300;
    let window_height = 420;
    
    let fixed_x = if x < 0 {
        0
    } else if x + window_width > screen_width {
        screen_width - window_width
    } else {
        x
    };
    
    let fixed_y = if y < 0 {
        0
    } else if y + window_height > screen_height {
        screen_height - window_height
    } else {
        y
    };
    
    (fixed_x, fixed_y)
}

// 检查位置是否需要修正
fn position_needs_correction(current_x: i32, current_y: i32, fixed_x: i32, fixed_y: i32) -> bool {
    current_x != fixed_x || current_y != fixed_y
}

// 防抖实现
impl DragDebouncer {
    fn new() -> Self {
        Self {
            last_move_time: Arc::new(Mutex::new(None)),
            pending_position: Arc::new(Mutex::new(None)),
        }
    }
    
    // 记录拖动事件，但不立即处理
    fn on_drag_move(&self, x: i32, y: i32) {
        if let Ok(mut last_time) = self.last_move_time.lock() {
            *last_time = Some(Instant::now());
        }
        if let Ok(mut pending_pos) = self.pending_position.lock() {
            *pending_pos = Some((x, y));
        }
    }
    
    // 检查是否应该处理拖动结束事件
    fn should_process_drag_end(&self, debounce_duration: Duration) -> Option<(i32, i32)> {
        if let (Ok(last_time), Ok(mut pending_pos)) = (self.last_move_time.lock(), self.pending_position.lock()) {
            if let (Some(time), Some(pos)) = (*last_time, *pending_pos) {
                if time.elapsed() >= debounce_duration {
                    *pending_pos = None; // 清除待处理位置
                    return Some(pos);
                }
            }
        }
        None
    }
}

// 获取窗口实际尺寸的函数（静默版本，减少日志输出）
#[cfg(target_os = "windows")]
fn get_actual_window_size(window: &tauri::WebviewWindow) -> (i32, i32) {
    match window.outer_size() {
        Ok(size) => (size.width as i32, size.height as i32),
        Err(_) => (300, 420) // 默认尺寸
    }
}

// 基于实际窗口尺寸的位置验证（只在需要时输出日志）
#[cfg(target_os = "windows")]
fn validate_position_with_actual_size(window: &tauri::WebviewWindow, x: i32, y: i32, log_changes: bool) -> (i32, i32) {
    let (work_left, work_top, work_right, work_bottom) = get_screen_work_area();
    let (actual_width, actual_height) = get_actual_window_size(window);
    
    // 严格的边界检查，使用实际窗口尺寸
    let fixed_x = x.max(work_left).min(work_right - actual_width);
    let fixed_y = y.max(work_top).min(work_bottom - actual_height);
    
    // 双重检查：确保窗口完全在工作区内
    let final_x = if fixed_x + actual_width > work_right {
        work_right - actual_width
    } else if fixed_x < work_left {
        work_left
    } else {
        fixed_x
    };
    
    let final_y = if fixed_y + actual_height > work_bottom {
        work_bottom - actual_height
    } else if fixed_y < work_top {
        work_top
    } else {
        fixed_y
    };
    
    // 只在需要时输出日志
    if log_changes && (final_x != x || final_y != y) {
        println!("位置修正: ({}, {}) -> ({}, {})", x, y, final_x, final_y);
        println!("实际窗口尺寸: {}x{}", actual_width, actual_height);
        println!("修正后窗口边界: 左={}, 上={}, 右={}, 下={}", 
            final_x, final_y, final_x + actual_width, final_y + actual_height);
    }
    
    (final_x, final_y)
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

// Tauri 命令：显示/隐藏主窗口
#[tauri::command]
async fn toggle_main_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        match window.is_visible() {
            Ok(true) => {
                let _ = window.hide();
            }
            Ok(false) => {
                let _ = window.show();
                let _ = window.set_focus();
            }
            Err(_) => {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
    }
    Ok(())
}

// Tauri 命令：只显示主窗口（不隐藏）
#[tauri::command]
async fn show_main_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }
    Ok(())
}

// Tauri 命令：最小化到托盘
#[tauri::command]
async fn minimize_to_tray(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }
    Ok(())
}

// Tauri 命令：从托盘恢复
#[tauri::command]
async fn restore_from_tray(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }
    Ok(())
}

// Tauri 命令：获取应用版本
#[tauri::command]
async fn get_app_version(app: tauri::AppHandle) -> Result<String, String> {
    Ok(app.package_info().version.to_string())
}




// Tauri 命令：退出应用
#[tauri::command]
async fn quit_app(app: tauri::AppHandle) -> Result<(), String> {
    app.exit(0);
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
        // 注意：设置窗口保持不透明，不应用透明度设置
        return Ok(());
    }

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
    .build()
    .map_err(|e| format!("创建设置窗口失败: {}", e))?;

    // 注意：设置窗口保持不透明，不应用透明度设置
    // 透明度设置只应用于主窗口

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
            set_todo_deadline,
            save_app_settings,
            load_app_settings,
            apply_opacity,
            save_window_position,
            load_window_position,
            toggle_main_window,
            show_main_window,
            quit_app,
            minimize_to_tray,
            restore_from_tray,
            get_app_version
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
                            let (fixed_x, fixed_y) = validate_and_fix_position(position.x, position.y);
                            
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
                            let (x, y) = get_top_right_position();
                            
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
                            let (x, y) = get_top_right_position();
                            let _ = window_clone.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                                x,
                                y,
                            }));
                        }
                    }
                });
                
                // 确保位置设置完成后再显示窗口
                let _ = window.show();
                let _ = window.set_focus();
                
                // 延迟应用透明度设置，确保窗口完全初始化
                if let Some(settings) = loaded_settings {
                    let window_for_opacity = window.clone();
                    let opacity_value = settings.opacity;
                    std::thread::spawn(move || {
                        std::thread::sleep(std::time::Duration::from_millis(100));
                        if let Err(e) = set_window_opacity(&window_for_opacity, opacity_value) {
                            println!("应用启动透明度失败: {}", e);
                        } else {
                            println!("成功应用启动透明度: {}", opacity_value);
                        }
                    });
                }

                // 在Tauri 2.x中处理窗口事件
                #[cfg(target_os = "windows")]
                {
                    let win_handle = window.clone();

                    // 创建拖动防抖器
                    let drag_debouncer = DragDebouncer::new();
                    
                    // 监听窗口事件
                    let app_handle_for_events = app.handle().clone();
                    let debouncer_for_events = drag_debouncer.clone();
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
                            // 检测窗口移动事件 - 使用防抖机制
                            tauri::WindowEvent::Moved(position) => {
                                // 记录拖动事件，但不立即处理
                                debouncer_for_events.on_drag_move(position.x, position.y);
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

                    // 定时器：处理 Win+D 恢复和拖动防抖
                    let win_handle2 = window.clone();
                    let app_handle_for_timer = app.handle().clone();
                    let debouncer_for_timer = drag_debouncer.clone();
                    std::thread::spawn(move || {
                        loop {
                            std::thread::sleep(std::time::Duration::from_millis(300)); // 更频繁检查防抖

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

                            // 检查是否需要处理拖动结束事件（500ms 防抖延迟）
                            if let Some((x, y)) = debouncer_for_timer.should_process_drag_end(Duration::from_millis(500)) {
                                println!("拖动结束，检查位置: ({}, {})", x, y);
                                
                                // 使用实际窗口尺寸验证并修正窗口位置
                                let (fixed_x, fixed_y) = validate_position_with_actual_size(&win_handle2, x, y, true);
                                
                                // 如果位置需要修正，调整窗口位置
                                if position_needs_correction(x, y, fixed_x, fixed_y) {
                                    let _ = win_handle2.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                                        x: fixed_x,
                                        y: fixed_y,
                                    }));
                                }
                                
                                // 保存最终位置
                                let app_handle = app_handle_for_timer.clone();
                                tauri::async_runtime::spawn(async move {
                                    let _ = save_window_position(app_handle, fixed_x, fixed_y).await;
                                });
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
