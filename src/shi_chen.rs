use chrono::{Datelike, Local, Timelike};
use lunar_rust::eight_char::EightCharRefHelper;
use lunar_rust::lunar::LunarRefHelper;
use lunar_rust::solar;
use lunar_rust::solar::{SolarRefHelper};

/// 获取当前时辰的序号 (子=1, 丑=2, ..., 亥=12)
pub fn get_current_shichen_num() -> u8 {
    let hour = Local::now().hour();
    match hour {
        23 | 0 => 1,   // 子
        1 | 2 => 2,    // 丑
        3 | 4 => 3,    // 寅
        5 | 6 => 4,    // 卯
        7 | 8 => 5,    // 辰
        9 | 10 => 6,   // 巳
        11 | 12 => 7,  // 午
        13 | 14 => 8,  // 未
        15 | 16 => 9,  // 申
        17 | 18 => 10, // 酉
        19 | 20 => 11, // 戌
        21 | 22 => 12, // 亥
        _ => unreachable!(),
    }
}

/// 获取当前季节 (1:春/木, 2:夏/火, 3:秋/金, 4:冬/水, 5:四季末/土)
/// 100% 专业版：基于真实的天文星历与二十四节气测算
pub fn get_current_season_pro() -> u8 {
    let now = Local::now();

    // 1. 构造精确到秒的公历时间对象
    let solar_date = solar::from_ymdhms(
        now.year() as i64,
        now.month() as i64,
        now.day() as i64,
        now.hour() as i64,
        now.minute() as i64,
        now.second() as i64,
    );

    // 2. 转换为干支农历对象
    let lunar_date = solar_date.get_lunar();

    // 3. 获取八字对象，并提取精准的“月建（地支）”
    // 此方法会自动计算当前时间是否已经过了具体的“交节”点（如立春、惊蛰）
    let ba_zi = lunar_date.get_eight_char();
    let month_zhi = ba_zi.get_month_zhi();

    // 4. 根据十二地支严格划分季节与五行
    // 寅卯(木), 巳午(火), 申酉(金), 亥子(水), 辰戌丑未(土)
    match month_zhi.as_str() {
        "寅" | "卯" => 1,       // 春，木旺
        "巳" | "午" => 2,       // 夏，火旺
        "申" | "酉" => 3,       // 秋，金旺
        "亥" | "子" => 4,       // 冬，水旺
        "辰" | "未" | "戌" | "丑" => 5, // 四季末，土旺
        _ => unreachable!("未知的月支: {}", month_zhi),
    }
}