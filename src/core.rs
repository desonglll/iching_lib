
#[cfg(test)]
mod tests {
    use crate::gua::{BaGua, ChongGua};
    use crate::wu_xing::WuXing;
    use crate::yao::Yao;
    use crate::yao::Yao::*;

    #[test]
    fn test_yao_xiang() {
        let qian = BaGua::from_number(1).unwrap();
        let yaoxiang_qian = qian.yao_xiang();
        assert_eq!(yaoxiang_qian, [Yang, Yang, Yang]);

        // 测试兑卦
        let dui = BaGua::from_number(2).unwrap();
        let yaoxiang_dui = dui.yao_xiang();
        assert_eq!(yaoxiang_dui, [Yang, Yang, Yin]);

        // 测试离卦
        let li = BaGua::from_number(3).unwrap();
        let yaoxiang_li = li.yao_xiang();
        assert_eq!(yaoxiang_li, [Yang, Yin, Yang]);

        // 测试震卦
        let zhen = BaGua::from_number(4).unwrap();
        let yaoxiang_zhen = zhen.yao_xiang();
        assert_eq!(yaoxiang_zhen, [Yang, Yin, Yin]);

        // 测试巽卦
        let xun = BaGua::from_number(5).unwrap();
        let yaoxiang_xun = xun.yao_xiang();
        assert_eq!(yaoxiang_xun, [Yin, Yang, Yang]);

        // 测试坎卦
        let kan = BaGua::from_number(6).unwrap();
        let yaoxiang_kan = kan.yao_xiang();
        assert_eq!(yaoxiang_kan, [Yin, Yang, Yin]);

        // 测试艮卦
        let r#gen = BaGua::from_number(7).unwrap();
        let yaoxiang_gen = r#gen.yao_xiang();
        assert_eq!(yaoxiang_gen, [Yin, Yin, Yang]);

        // 测试坤卦
        let kun = BaGua::from_number(8).unwrap();
        let yaoxiang_kun = kun.yao_xiang();
        assert_eq!(yaoxiang_kun, [Yin, Yin, Yin]);
    }

    #[test]
    fn test_bagua_from_number_valid() {
        assert_eq!(BaGua::from_number(1), Some(BaGua::Qian));
        assert_eq!(BaGua::from_number(2), Some(BaGua::Dui));
        assert_eq!(BaGua::from_number(3), Some(BaGua::Li));
        assert_eq!(BaGua::from_number(4), Some(BaGua::Zhen));
        assert_eq!(BaGua::from_number(5), Some(BaGua::Xun));
        assert_eq!(BaGua::from_number(6), Some(BaGua::Kan));
        assert_eq!(BaGua::from_number(7), Some(BaGua::Gen));
        assert_eq!(BaGua::from_number(8), Some(BaGua::Kun));
    }

    #[test]
    fn test_bagua_from_number_invalid() {
        assert_eq!(BaGua::from_number(0), None);
        assert_eq!(BaGua::from_number(9), None);
        assert_eq!(BaGua::from_number(100), None);
    }

    #[test]
    fn test_wuxing() {
        // 测试金属性
        assert_eq!(BaGua::Qian.wu_xing(), WuXing::Jin);
        assert_eq!(BaGua::Dui.wu_xing(), WuXing::Jin);

        // 测试木属性
        assert_eq!(BaGua::Zhen.wu_xing(), WuXing::Mu);
        assert_eq!(BaGua::Xun.wu_xing(), WuXing::Mu);

        // 测试水属性
        assert_eq!(BaGua::Kan.wu_xing(), WuXing::Shui);

        // 测试火属性
        assert_eq!(BaGua::Li.wu_xing(), WuXing::Huo);

        // 测试土属性
        assert_eq!(BaGua::Gen.wu_xing(), WuXing::Tu);
        assert_eq!(BaGua::Kun.wu_xing(), WuXing::Tu);
    }

    #[test]
    fn test_chonggua_new() {
        let chonggua = ChongGua::new(BaGua::Qian, BaGua::Kun);
        assert_eq!(chonggua.shang, BaGua::Qian);
        assert_eq!(chonggua.xia, BaGua::Kun);
    }

    #[test]
    fn test_chonggua_ti_yong() {
        let chonggua = ChongGua::new(BaGua::Qian, BaGua::Kun);

        // 动爻在下卦（1-3爻），下卦为用卦，上卦为体卦
        assert_eq!(chonggua.ti_yong(1), Some((BaGua::Qian, BaGua::Kun)));
        assert_eq!(chonggua.ti_yong(2), Some((BaGua::Qian, BaGua::Kun)));
        assert_eq!(chonggua.ti_yong(3), Some((BaGua::Qian, BaGua::Kun)));

        // 动爻在上卦（4-6爻），上卦为用卦，下卦为体卦
        assert_eq!(chonggua.ti_yong(4), Some((BaGua::Kun, BaGua::Qian)));
        assert_eq!(chonggua.ti_yong(5), Some((BaGua::Kun, BaGua::Qian)));
        assert_eq!(chonggua.ti_yong(6), Some((BaGua::Kun, BaGua::Qian)));

        // 无效动爻
        assert_eq!(chonggua.ti_yong(0), None);
        assert_eq!(chonggua.ti_yong(7), None);
        assert_eq!(chonggua.ti_yong(100), None);
    }

    #[test]
    fn test_chonggua_yao_xiang() {
        // 测试乾坤卦的六爻
        let chonggua = ChongGua::new(BaGua::Qian, BaGua::Kun);
        let expected = [Yin, Yin, Yin, Yang, Yang, Yang]; // 下卦坤，上卦乾
        assert_eq!(chonggua.yao_xiang(), expected);

        // 测试离震卦
        let chonggua2 = ChongGua::new(BaGua::Li, BaGua::Zhen);
        let expected2 = [Yang, Yin, Yin, Yang, Yin, Yang]; // 下卦震，上卦离
        assert_eq!(chonggua2.yao_xiang(), expected2);

        // 测试兑巽卦
        let chonggua3 = ChongGua::new(BaGua::Dui, BaGua::Xun);
        let expected3 = [Yin, Yang, Yang, Yang, Yang, Yin]; // 下卦巽，上卦兑
        assert_eq!(chonggua3.yao_xiang(), expected3);
    }

    #[test]
    fn test_enum_equality() {
        // 测试 Yao 枚举
        assert_eq!(Yao::Yin, Yao::Yin);
        assert_eq!(Yao::Yang, Yao::Yang);
        assert_ne!(Yao::Yin, Yao::Yang);

        // 测试 BaGua 枚举
        assert_eq!(BaGua::Qian, BaGua::Qian);
        assert_ne!(BaGua::Qian, BaGua::Kun);

        // 测试 WuXing 枚举
        assert_eq!(WuXing::Jin, WuXing::Jin);
        assert_ne!(WuXing::Jin, WuXing::Mu);
    }

    #[test]
    fn test_chonggua_multiple_combinations() {
        // 测试多种重卦组合
        let combinations = vec![
            (BaGua::Qian, BaGua::Qian), // 乾乾
            (BaGua::Kun, BaGua::Kun),   // 坤坤
            (BaGua::Li, BaGua::Kan),    // 离坎
            (BaGua::Zhen, BaGua::Xun),  // 震巽
        ];

        for (shang, xia) in combinations {
            let chonggua = ChongGua::new(shang, xia);
            assert_eq!(chonggua.shang, shang);
            assert_eq!(chonggua.xia, xia);
        }
    }
}
