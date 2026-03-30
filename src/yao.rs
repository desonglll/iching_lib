use std::fmt::{Display, Formatter, Write};

/// 爻
///
/// 奇數即為陽數，偶數則為陰數
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Yao {
    Yin,
    Yang,
}

impl Display for Yao {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Yao::Yin => f.write_char('\u{268A}'),
            Yao::Yang => f.write_char('\u{268B}'),
        }
    }
}

/// 根据数字获取动爻
///
/// 逻辑：(上卦数 + 下卦数 + 时辰数) % 6
/// 如果输入了 shi_chen (1-12)，则参与计算；否则仅计算两数之和
pub fn get_dong_yao(num1: u8, num2: u8, shi_chen: Option<u8>) -> u8 {
    // 如果有时间信息，则累加；没有则为 0
    let time_val = shi_chen.unwrap_or(0);
    let sum = num1 + num2 + time_val;

    let remainder = sum % 6;
    if remainder == 0 { 6 } else { remainder }
}
