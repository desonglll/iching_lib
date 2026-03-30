use chrono::{Local, Timelike, Datelike};

/// 获取当前时辰的序号 (子=1, 丑=2, ..., 亥=12)
pub fn get_current_shichen_num() -> u8 {
    let hour = Local::now().hour();
    match hour {
        23 | 0 => 1,  // 子
        1 | 2 => 2,   // 丑
        3 | 4 => 3,   // 寅
        5 | 6 => 4,   // 卯
        7 | 8 => 5,   // 辰
        9 | 10 => 6,  // 巳
        11 | 12 => 7, // 午
        13 | 14 => 8, // 未
        15 | 16 => 9, // 申
        17 | 18 => 10, // 酉
        19 | 20 => 11, // 戌
        21 | 22 => 12, // 亥
        _ => unreachable!(),
    }
}

/// 获取当前季节 (1:春, 2:夏, 3:秋, 4:冬, 5:四季末/土旺)
/// 简化版：根据月份划分
pub fn get_current_season() -> u8 {
    let month = Local::now().month();
    match month {
        3 | 4 | 5 => 1,  // 春（木旺）
        6 | 7 | 8 => 2,  // 夏（火旺）
        9 | 10 | 11 => 3, // 秋（金旺）
        12 | 1 | 2 => 4,  // 冬（水旺）
        _ => 5,
    }
}