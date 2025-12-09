#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{SystemParametersInfoW, SPI_GETWORKAREA};

// 获取屏幕工作区信息
#[cfg(target_os = "windows")]
pub fn get_screen_work_area() -> (i32, i32, i32, i32) {
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
pub fn get_top_right_position() -> (i32, i32) {
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
pub fn get_top_right_position() -> (i32, i32) {
    // 在非Windows平台上，使用默认位置
    (1000, 50)
}

// 严格检查并修正窗口位置，确保窗口完全在工作区内
#[cfg(target_os = "windows")]
pub fn validate_and_fix_position(x: i32, y: i32) -> (i32, i32) {
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
pub fn validate_and_fix_position(x: i32, y: i32) -> (i32, i32) {
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

