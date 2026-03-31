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
            Yao::Yin => f.write_char('\u{268B}'),
            Yao::Yang => f.write_char('\u{268A}'),
        }
    }
}

/// 根据数字获取动爻
///
/// 逻辑：(上卦数 + 下卦数 + 时辰数) % 6
/// 如果输入了 shi_chen (1-12)，则参与计算；否则仅计算两数之和
pub fn get_dong_yao(num1: u8, num2: u8, shi_chen: Option<u8>) -> u8 {
    let sum;
    if let Some(shi_chen) = shi_chen {
        if (1..=12).contains(&shi_chen){
            sum = num1 + num2 + shi_chen
        }
        else {
            sum = num1 + num2
        }
    }

    else {
        sum = num1 + num2
    }

    let remainder = sum % 6;
    if remainder == 0 { 6 } else { remainder }
}

#[cfg(test)]
mod tests {
    use crate::yao::get_dong_yao;

    #[test]
    fn test_get_dong_yao() {
        // 测试不包含时辰的情况
        assert_eq!(get_dong_yao(3, 4, None), 1); // (3+4)%6 = 1
        assert_eq!(get_dong_yao(5, 7, None), 6); // (5+7)%6 = 0 -> 6
        assert_eq!(get_dong_yao(0, 0, None), 6); // (0+0)%6 = 0 -> 6
        
        // 测试包含有效时辰的情况 (1-12)
        assert_eq!(get_dong_yao(1, 1, Some(4)), 6); // (1+1+4)%6 = 0 -> 6
        assert_eq!(get_dong_yao(2, 3, Some(1)), 6); // (2+3+1)%6 = 0 -> 6
        assert_eq!(get_dong_yao(1, 2, Some(3)), 6); // (1+2+3)%6 = 0 -> 6
        assert_eq!(get_dong_yao(2, 2, Some(2)), 6); // (2+2+2)%6 = 0 -> 6
        assert_eq!(get_dong_yao(1, 1, Some(12)), 2); // (1+1+12)%6 = 2

        // 测试包含无效时辰的情况 (不在1-12范围内)
        assert_eq!(get_dong_yao(3, 4, Some(0)), 1);  // (3+4)%6 = 1, 0无效
        assert_eq!(get_dong_yao(3, 4, Some(13)), 1); // (3+4)%6 = 1, 13无效
    }
}