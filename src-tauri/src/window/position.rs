#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{SystemParametersInfoW, SPI_GETWORKAREA};

// 获取包含指定点的显示器工作区
#[cfg(target_os = "windows")]
pub fn get_monitor_work_area_at_point(x: i32, y: i32) -> (i32, i32, i32, i32) {
    use windows::Win32::Foundation::POINT;
    use windows::Win32::Graphics::Gdi::{MonitorFromPoint, GetMonitorInfoW, MONITORINFO, MONITOR_DEFAULTTONEAREST};
    
    unsafe {
        let point = POINT { x, y };
        let monitor = MonitorFromPoint(point, MONITOR_DEFAULTTONEAREST);
        
        let mut monitor_info = MONITORINFO {
            cbSize: std::mem::size_of::<MONITORINFO>() as u32,
            ..Default::default()
        };
        
        if GetMonitorInfoW(monitor, &mut monitor_info).as_bool() {
            let work = monitor_info.rcWork;
            println!("显示器工作区 at ({}, {}): left={}, top={}, right={}, bottom={}", 
                x, y, work.left, work.top, work.right, work.bottom);
            (work.left, work.top, work.right, work.bottom)
        } else {
            // 回退到主显示器
            get_primary_monitor_work_area()
        }
    }
}

// 获取主显示器工作区信息
#[cfg(target_os = "windows")]
pub fn get_primary_monitor_work_area() -> (i32, i32, i32, i32) {
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
            println!("主显示器工作区: left={}, top={}, right={}, bottom={}", 
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
    let (work_left, work_top, work_right, work_bottom) = get_primary_monitor_work_area();
    
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

// 获取屏幕中心位置
#[cfg(target_os = "windows")]
pub fn get_center_position() -> (i32, i32) {
    let (work_left, work_top, work_right, work_bottom) = get_primary_monitor_work_area();
    
    // 窗口尺寸
    let window_width = 300;
    let window_height = 420;
    
    // 工作区的实际宽度和高度
    let work_width = work_right - work_left;
    let work_height = work_bottom - work_top;
    
    // 计算中心位置
    let x = work_left + (work_width - window_width) / 2;
    let y = work_top + (work_height - window_height) / 2;
    
    println!("屏幕中心位置: ({}, {})", x, y);
    
    (x, y)
}

#[cfg(not(target_os = "windows"))]
pub fn get_top_right_position() -> (i32, i32) {
    // 在非Windows平台上，使用默认位置
    (1000, 50)
}

#[cfg(not(target_os = "windows"))]
pub fn get_center_position() -> (i32, i32) {
    // 在非Windows平台上，使用默认中心位置
    (810, 330)
}

// 检查窗口位置是否有效（支持多显示器）
// 如果窗口溢出屏幕边界，则回弹到边界对齐
#[cfg(target_os = "windows")]
pub fn validate_and_fix_position(x: i32, y: i32) -> (i32, i32) {
    // 窗口尺寸
    let window_width = 300;
    let window_height = 420;
    
    // 计算窗口中心点
    let center_x = x + window_width / 2;
    let center_y = y + window_height / 2;
    
    // 获取窗口中心点所在显示器的工作区
    let (work_left, work_top, work_right, work_bottom) = get_monitor_work_area_at_point(center_x, center_y);
    
    let mut fixed_x = x;
    let mut fixed_y = y;
    let mut needs_fix = false;
    
    // 检查并修正左边界：如果窗口左侧超出屏幕左侧，则对齐到左边界
    if x < work_left {
        fixed_x = work_left;
        needs_fix = true;
    }
    
    // 检查并修正右边界：如果窗口右侧超出屏幕右侧，则对齐到右边界
    if x + window_width > work_right {
        fixed_x = work_right - window_width;
        needs_fix = true;
    }
    
    // 检查并修正上边界：如果窗口顶部超出屏幕顶部，则对齐到上边界
    if y < work_top {
        fixed_y = work_top;
        needs_fix = true;
    }
    
    // 检查并修正下边界：如果窗口底部超出屏幕底部，则对齐到下边界
    if y + window_height > work_bottom {
        fixed_y = work_bottom - window_height;
        needs_fix = true;
    }
    
    if needs_fix {
        println!("窗口溢出边界，回弹对齐: ({}, {}) -> ({}, {})", x, y, fixed_x, fixed_y);
        println!("工作区边界: 左={}, 上={}, 右={}, 下={}", 
            work_left, work_top, work_right, work_bottom);
    } else {
        println!("窗口位置有效（在边界内）: ({}, {})", x, y);
    }
    
    (fixed_x, fixed_y)
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

