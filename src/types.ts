export interface Todo {
  text: string;
  completed: boolean;
  createdAt: number; // Unix时间戳（秒）
  deadline?: number; // 截止时间，Unix时间戳（秒），可选
}

export interface DateInfo {
  solar_date: string;    // 公历日期
  lunar_date: string;    // 农历日期
  weekday: string;       // 星期
  lunar_year: string;    // 农历年份
  lunar_month: string;   // 农历月份
  lunar_day: string;     // 农历日期
}