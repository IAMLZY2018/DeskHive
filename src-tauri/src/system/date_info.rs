use chrono::{Local, Datelike};

use crate::models::DateInfo;

// 获取当前日期信息（公历和农历）
pub fn get_current_date_info() -> DateInfo {
    let now = Local::now();
    
    // 公历信息
    let solar_date = now.format("%Y年%m月%d日").to_string();
    let weekday = match now.weekday() {
        chrono::Weekday::Mon => "星期一".to_string(),
        chrono::Weekday::Tue => "星期二".to_string(),
        chrono::Weekday::Wed => "星期三".to_string(),
        chrono::Weekday::Thu => "星期四".to_string(),
        chrono::Weekday::Fri => "星期五".to_string(),
        chrono::Weekday::Sat => "星期六".to_string(),
        chrono::Weekday::Sun => "星期日".to_string(),
    };
    
    // 简化版的农历信息（作为示例）
    // 在实际应用中，这里可以集成真正的农历计算库
    let (lunar_year, lunar_month, lunar_day, lunar_date) = get_sample_lunar_info(&now);
    
    DateInfo {
        solar_date,
        lunar_date,
        weekday,
        lunar_year,
        lunar_month,
        lunar_day,
    }
}

// 获取示例农历信息（简化版）
fn get_sample_lunar_info(date: &chrono::DateTime<chrono::Local>) -> (String, String, String, String) {
    // 这里使用一个简化的映射，实际应用中可以更换为真正的农历计算
    let _year = 2024; // 使用下划线前缀来避免警告
    let month = date.month();
    let day = date.day();
    
    // 根据月份和日期生成一个简单的农历示例
    let lunar_year_str = "甲辰年".to_string();
    let lunar_month_str = match month {
        1..=3 => "正月".to_string(),
        4..=6 => "四月".to_string(),
        7..=9 => "七月".to_string(),
        _ => "十月".to_string(),
    };
    
    let lunar_day_str = match day {
        1..=10 => format!("初{}", get_chinese_number(day as i32)),
        11..=20 => format!("十{}", if day == 10 { "".to_string() } else { get_chinese_number((day - 10) as i32) }),
        21..=31 => format!("廿{}", get_chinese_number((day - 20) as i32)),
        _ => format!("{}日", day),
    };
    
    let full_lunar_date = format!("{}{}{}", lunar_year_str, lunar_month_str, lunar_day_str);
    
    (lunar_year_str, lunar_month_str, lunar_day_str, full_lunar_date)
}

// 获取中文数字
fn get_chinese_number(num: i32) -> String {
    match num {
        1 => "一".to_string(),
        2 => "二".to_string(),
        3 => "三".to_string(),
        4 => "四".to_string(),
        5 => "五".to_string(),
        6 => "六".to_string(),
        7 => "七".to_string(),
        8 => "八".to_string(),
        9 => "九".to_string(),
        10 => "十".to_string(),
        _ => num.to_string(),
    }
}

// Tauri 命令：获取当前日期信息
#[tauri::command]
pub async fn get_current_date() -> Result<DateInfo, String> {
    Ok(get_current_date_info())
}