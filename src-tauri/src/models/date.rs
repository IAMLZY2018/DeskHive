use serde::{Deserialize, Serialize};

// 日期信息结构
#[derive(Serialize, Deserialize, Clone)]
pub struct DateInfo {
    pub solar_date: String,      // 公历日期，格式：2024年8月28日
    pub lunar_date: String,      // 农历日期，格式：甲辰年七月廿五
    pub weekday: String,         // 星期，格式：星期三
    pub lunar_year: String,      // 农历年份，格式：甲辰年（龙年）
    pub lunar_month: String,     // 农历月份，格式：七月
    pub lunar_day: String,       // 农历日期，格式：廿五
}