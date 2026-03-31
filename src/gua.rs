use crate::wu_xing::WuXing;
use crate::yao::Yao;
use std::fmt;
use std::fmt::{Display, Formatter};

/// 八卦
///
/// 乾一、兌二、離三、震四、巽五、坎六、艮七、坤八
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaGua {
    Qian,
    Dui,
    Li,
    Zhen,
    Xun,
    Kan,
    Gen,
    Kun,
}

impl Display for BaGua {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BaGua::Qian => f.write_str("乾"),
            BaGua::Dui => f.write_str("兑"),
            BaGua::Li => f.write_str("离"),
            BaGua::Zhen => f.write_str("震"),
            BaGua::Xun => f.write_str("巽"),
            BaGua::Kan => f.write_str("坎"),
            BaGua::Gen => f.write_str("艮"),
            BaGua::Kun => f.write_str("坤"),
        }
    }
}

impl BaGua {
    /// 根据数字（1-8）获得对应的八卦
    ///
    /// 乾一、兑二、离三、震四、巽五、坎六、艮七、坤八
    pub fn from_number(num: u32) -> Option<Self> {
        if !(1..=8).contains(&num) {
            return None;
        };
        let index = num - 1;

        let r = match index {
            0 => BaGua::Qian, // 乾1
            1 => BaGua::Dui, // 兑2
            2 => BaGua::Li, // 离3
            3 => BaGua::Zhen, // 震4
            4 => BaGua::Xun, // 巽5
            5 => BaGua::Kan, // 坎6
            6 => BaGua::Gen, // 艮7
            7 => BaGua::Kun, // 坤8
            _ => unreachable!(),
        };
        Some(r)
    }

    /// 获取八卦对应的 Unicode 符号 (\u{2630} - \u{2637})
    ///
    /// 乾一、兑二、离三、震四、巽五、坎六、艮七、坤八
    pub fn unicode_symbol(&self) -> char {
        match self {
            BaGua::Qian => '\u{2630}',
            BaGua::Dui => '\u{2631}',
            BaGua::Li => '\u{2632}',
            BaGua::Zhen => '\u{2633}',
            BaGua::Xun => '\u{2634}',
            BaGua::Kan => '\u{2635}',
            BaGua::Gen => '\u{2636}',
            BaGua::Kun => '\u{2637}',
        }
    }
    /// 获取它的阴阳爻象（初爻、二爻、三爻）
    /// 从下到上（初爻 -> 三爻）
    pub fn yao_xiang(&self) -> [Yao; 3] {
        use crate::yao::Yao::*;
        match self {
            BaGua::Qian => [Yang, Yang, Yang],
            BaGua::Dui => [Yang, Yang, Yin],
            BaGua::Li => [Yang, Yin, Yang],
            BaGua::Zhen => [Yang, Yin, Yin],
            BaGua::Xun => [Yin, Yang, Yang],
            BaGua::Kan => [Yin, Yang, Yin],
            BaGua::Gen => [Yin, Yin, Yang],
            BaGua::Kun => [Yin, Yin, Yin],
        }
    }

    /// 获取八卦对应的先天数（伏羲数）
    pub fn xian_tian_shu(&self) -> u8 {
        match self {
            BaGua::Qian => 1,
            BaGua::Dui => 2,
            BaGua::Li => 3,
            BaGua::Zhen => 4,
            BaGua::Xun => 5,
            BaGua::Kan => 6,
            BaGua::Gen => 7,
            BaGua::Kun => 8,
        }
    }

    /// 获取八卦对应的后天九宫数（洛书数）
    pub fn jiu_gong_shu(&self) -> u8 {
        match self {
            BaGua::Kan => 1,
            BaGua::Kun => 2,
            BaGua::Zhen => 3,
            BaGua::Xun => 4,
            BaGua::Qian => 6,
            BaGua::Dui => 7,
            BaGua::Gen => 8,
            BaGua::Li => 9,
        }
    }

    /// 获取八卦对应的五行
    ///
    /// 乾、兑 属金
    /// 震、巽 属木
    /// 坎 属水
    /// 离 属火
    /// 艮、坤 属土
    pub fn wu_xing(&self) -> WuXing {
        match self {
            BaGua::Qian | BaGua::Dui => WuXing::Jin,
            BaGua::Zhen | BaGua::Xun => WuXing::Mu,
            BaGua::Kan => WuXing::Shui,
            BaGua::Li => WuXing::Huo,
            BaGua::Gen | BaGua::Kun => WuXing::Tu,
        }
    }

    /// 根据三个爻的组合获取对应的八卦
    /// 这里的数组顺序应当与 `yao_xiang` 方法保持一致：[初爻, 二爻, 三爻]
    pub fn from_yao_xiang(yao: [Yao; 3]) -> Self {
        use crate::yao::Yao::*;
        match yao {
            [Yang, Yang, Yang] => BaGua::Qian,
            [Yang, Yang, Yin] => BaGua::Dui,
            [Yang, Yin, Yang] => BaGua::Li,
            [Yang, Yin, Yin] => BaGua::Zhen,
            [Yin, Yang, Yang] => BaGua::Xun,
            [Yin, Yang, Yin] => BaGua::Kan,
            [Yin, Yin, Yang] => BaGua::Gen,
            [Yin, Yin, Yin] => BaGua::Kun,
        }
    }
}

/// 重卦（六十四卦）
///
/// 上卦（外卦）
/// 下卦（内卦）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChongGua {
    pub shang: BaGua,
    pub xia: BaGua,
}

impl Display for ChongGua {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "上卦（外卦）：{}\n下卦（内卦）：{}",
            self.shang, self.xia
        ))
    }
}

impl ChongGua {
    /// 组合上下卦
    pub fn new(shang: BaGua, xia: BaGua) -> Self {
        ChongGua { shang, xia }
    }

    /// 体卦、用卦
    ///
    /// 核心逻辑：根据“动爻”的位置（1到6），区分出【体卦】和【用卦】
    /// 规则：有动爻所在的单卦叫“用卦”（代表事件、客体），没有动爻的叫“体卦”（代表自己、主体）。
    /// 返回值：(体卦, 用卦)
    pub fn ti_yong(&self, dong_yao_index: u8) -> Option<(BaGua, BaGua)> {
        match dong_yao_index {
            1..=3 => Some((self.shang, self.xia)),
            4..=6 => Some((self.xia, self.shang)),
            _ => None,
        }
    }

    /// 从下到上（1~6爻）
    pub fn yao_xiang(&self) -> [Yao; 6] {
        let mut result = [Yao::Yin; 6];

        let xia = self.xia.yao_xiang();
        let shang = self.shang.yao_xiang();

        result[0..3].copy_from_slice(&xia);
        result[3..6].copy_from_slice(&shang);

        result
    }

    /// 获取互卦
    /// 逻辑：
    /// 互下卦 = 本卦的 2, 3, 4 爻
    /// 互上卦 = 本卦的 3, 4, 5 爻
    pub fn hu_gua(&self) -> Self {
        let yaos = self.yao_xiang(); // 获取 [初, 二, 三, 四, 五, 上] 爻

        // 互下卦：取本卦的 2, 3, 4 爻 (索引 1, 2, 3)
        let xia_hu = BaGua::from_yao_xiang([yaos[1], yaos[2], yaos[3]]);

        // 互上卦：取本卦的 3, 4, 5 爻 (索引 2, 3, 4)
        let shang_hu = BaGua::from_yao_xiang([yaos[2], yaos[3], yaos[4]]);

        ChongGua::new(shang_hu, xia_hu)
    }

    /// 错卦：阴阳全变（阳变阴，阴变阳）
    /// 代表：事物的背面、潜在的对立力量
    pub fn cuo_gua(&self) -> Self {
        let mut yaos = self.yao_xiang(); // [初, 二, 三, 四, 五, 上]

        for yao in yaos.iter_mut() {
            *yao = match *yao {
                Yao::Yang => Yao::Yin,
                Yao::Yin => Yao::Yang,
            };
        }

        let xia = BaGua::from_yao_xiang([yaos[0], yaos[1], yaos[2]]);
        let shang = BaGua::from_yao_xiang([yaos[3], yaos[4], yaos[5]]);

        ChongGua::new(shang, xia)
    }

    /// 综卦：上下倒置（初爻变上爻，二爻变五爻...）
    /// 代表：换个视角看问题，立场的转换
    pub fn zong_gua(&self) -> Self {
        let mut yaos = self.yao_xiang();
        // 将 [1, 2, 3, 4, 5, 6] 翻转为 [6, 5, 4, 3, 2, 1]
        yaos.reverse();

        let xia = BaGua::from_yao_xiang([yaos[0], yaos[1], yaos[2]]);
        let shang = BaGua::from_yao_xiang([yaos[3], yaos[4], yaos[5]]);

        ChongGua::new(shang, xia)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiuShiSiGua {
    // (1) 坤宮第一列 (內卦為坤)
    KunWeiDi,   // 2. 坤 (序号 01)
    ShanDiBo,   // 23. 剥 (序号 02)
    ShuiDiBi,   // 8. 比 (序号 03)
    FengDiGuan, // 20. 观 (序号 04)
    LeiDiYu,    // 16. 豫 (序号 05)
    HuoDiJin,   // 35. 晋 (序号 06)
    ZeDiCui,    // 45. 萃 (序号 07)
    TianDiPi,   // 12. 否 (序号 08)

    // (9) 艮宮第二列 (內卦為艮)
    DiShanQian,     // 15. 谦 (序号 09)
    GenWeiShan,     // 52. 艮 (序号 10)
    ShuiShanJian,   // 39. 蹇 (序号 11)
    FengShanJian,   // 53. 渐 (序号 12)
    LeiShanXiaoGuo, // 62. 小过 (序号 13)
    HuoShanLv,      // 56. 旅 (序号 14)
    ZeShanXian,     // 31. 咸 (序号 15)
    TianShanDun,    // 33. 遯 (序号 16)

    // (17) 坎宮第三列 (內卦為坎)
    DiShuiShi,    // 7. 师 (序号 17)
    ShanShuiMeng, // 4. 蒙 (序号 18)
    KanWeiShui,   // 29. 坎 (序号 19)
    FengShuiHuan, // 59. 涣 (序号 20)
    LeiShuiXie,   // 40. 解 (序号 21)
    HuoShuiWeiJi, // 64. 未济 (序号 22)
    ZeShuiKun,    // 47. 困 (序号 23)
    TianShuiSong, // 6. 讼 (序号 24)

    // (25) 巽宮第四列 (內卦為巽)
    DiFengSheng,  // 46. 升 (序号 25)
    ShanFengGu,   // 18. 蛊 (序号 26)
    ShuiFengJing, // 48. 井 (序号 27)
    XunWeiFeng,   // 57. 巽 (序号 28)
    LeiFengHeng,  // 32. 恒 (序号 29)
    HuoFengDing,  // 50. 鼎 (序号 30)
    ZeFengDaGuo,  // 28. 大过 (序号 31)
    TianFengGou,  // 44. 姤 (序号 32)

    // (33) 震宮第五列 (內卦為震)
    DiLeiFu,       // 24. 复 (序号 33)
    ShanLeiYi,     // 27. 颐 (序号 34)
    ShuiLeiTun,    // 3. 屯 (序号 35)
    FengLeiYi,     // 42. 益 (序号 36)
    ZhenWeiLei,    // 51. 震 (序号 37)
    HuoLeiShiHe,   // 21. 噬嗑 (序号 38)
    ZeLeiSui,      // 17. 随 (序号 39)
    TianLeiWuWang, // 25. 无妄 (序号 40)

    // (41) 離宮第六列 (內卦為離)
    DiHuoMingYi,    // 36. 明夷 (序号 41)
    ShanHuoBi,      // 22. 贲 (序号 42)
    ShuiHuoJiJi,    // 63. 既济 (序号 43)
    FengHuoJiaRen,  // 37. 家人 (序号 44)
    LeiHuoFeng,     // 55. 丰 (序号 45)
    LiWeiHuo,       // 30. 离 (序号 46)
    ZeHuoGe,        // 49. 革 (序号 47)
    TianHuoTongRen, // 13. 同人 (序号 48)

    // (49) 兌宮第七列 (內卦為兌)
    DiZeLin,       // 19. 临 (序号 49)
    ShanZeSun,     // 41. 损 (序号 50)
    ShuiZeJie,     // 60. 节 (序号 51)
    FengZeZhongFu, // 61. 中孚 (序号 52)
    LeiZeGuiMei,   // 54. 归妹 (序号 53)
    HuoZeKui,      // 38. 睽 (序号 54)
    DuiWeiZe,      // 58. 兑 (序号 55)
    TianZeLv,      // 10. 履 (序号 56)

    // (57) 乾宮第八列 (內卦為乾)
    DiTianTai,       // 11. 泰 (序号 57)
    ShanTianDaXu,    // 26. 大畜 (序号 58)
    ShuiTianXu,      // 5. 需 (序号 59)
    FengTianXiaoXu,  // 9. 小畜 (序号 60)
    LeiTianDaZhuang, // 34. 大壮 (序号 61)
    HuoTianDaYou,    // 14. 大有 (序号 62)
    ZeTianGuai,      // 43. 夬 (序号 63)
    QianWeiTian,     // 1. 乾 (序号 64)
}

impl Display for LiuShiSiGua {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = match self {
            // 第一列：內卦為坤
            LiuShiSiGua::KunWeiDi => "坤為地",
            LiuShiSiGua::ShanDiBo => "山地剝",
            LiuShiSiGua::ShuiDiBi => "水地比",
            LiuShiSiGua::FengDiGuan => "風地觀",
            LiuShiSiGua::LeiDiYu => "雷地豫",
            LiuShiSiGua::HuoDiJin => "火地晉",
            LiuShiSiGua::ZeDiCui => "澤地萃",
            LiuShiSiGua::TianDiPi => "天地否",

            // 第二列：內卦為艮
            LiuShiSiGua::DiShanQian => "地山謙",
            LiuShiSiGua::GenWeiShan => "艮為山",
            LiuShiSiGua::ShuiShanJian => "水山蹇",
            LiuShiSiGua::FengShanJian => "風山漸",
            LiuShiSiGua::LeiShanXiaoGuo => "雷山小過",
            LiuShiSiGua::HuoShanLv => "火山旅",
            LiuShiSiGua::ZeShanXian => "澤山咸",
            LiuShiSiGua::TianShanDun => "天山遯",

            // 第三列：內卦為坎
            LiuShiSiGua::DiShuiShi => "地水師",
            LiuShiSiGua::ShanShuiMeng => "山水蒙",
            LiuShiSiGua::KanWeiShui => "坎為水",
            LiuShiSiGua::FengShuiHuan => "風水渙",
            LiuShiSiGua::LeiShuiXie => "雷水解",
            LiuShiSiGua::HuoShuiWeiJi => "火水未濟",
            LiuShiSiGua::ZeShuiKun => "澤水困",
            LiuShiSiGua::TianShuiSong => "天水訟",

            // 第四列：內卦為巽
            LiuShiSiGua::DiFengSheng => "地風升",
            LiuShiSiGua::ShanFengGu => "山風蠱",
            LiuShiSiGua::ShuiFengJing => "水風井",
            LiuShiSiGua::XunWeiFeng => "巽為風",
            LiuShiSiGua::LeiFengHeng => "雷風恆",
            LiuShiSiGua::HuoFengDing => "火風鼎",
            LiuShiSiGua::ZeFengDaGuo => "澤風大過",
            LiuShiSiGua::TianFengGou => "天風姤",

            // 第五列：內卦為震
            LiuShiSiGua::DiLeiFu => "地雷復",
            LiuShiSiGua::ShanLeiYi => "山雷頤",
            LiuShiSiGua::ShuiLeiTun => "水雷屯",
            LiuShiSiGua::FengLeiYi => "風雷益",
            LiuShiSiGua::ZhenWeiLei => "震為雷",
            LiuShiSiGua::HuoLeiShiHe => "火雷噬嗑",
            LiuShiSiGua::ZeLeiSui => "澤雷隨",
            LiuShiSiGua::TianLeiWuWang => "天雷无妄",

            // 第六列：內卦為離
            LiuShiSiGua::DiHuoMingYi => "地火明夷",
            LiuShiSiGua::ShanHuoBi => "山火賁",
            LiuShiSiGua::ShuiHuoJiJi => "水火既濟",
            LiuShiSiGua::FengHuoJiaRen => "風火家人",
            LiuShiSiGua::LeiHuoFeng => "雷火豐",
            LiuShiSiGua::LiWeiHuo => "離為火",
            LiuShiSiGua::ZeHuoGe => "澤火革",
            LiuShiSiGua::TianHuoTongRen => "天火同人",

            // 第七列：內卦為兌
            LiuShiSiGua::DiZeLin => "地澤臨",
            LiuShiSiGua::ShanZeSun => "山澤損",
            LiuShiSiGua::ShuiZeJie => "水澤節",
            LiuShiSiGua::FengZeZhongFu => "風澤中孚",
            LiuShiSiGua::LeiZeGuiMei => "雷澤歸妹",
            LiuShiSiGua::HuoZeKui => "火澤睽",
            LiuShiSiGua::DuiWeiZe => "兌為澤",
            LiuShiSiGua::TianZeLv => "天澤履",

            // 第八列：內卦為乾
            LiuShiSiGua::DiTianTai => "地天泰",
            LiuShiSiGua::ShanTianDaXu => "山天大畜",
            LiuShiSiGua::ShuiTianXu => "水天需",
            LiuShiSiGua::FengTianXiaoXu => "風天小畜",
            LiuShiSiGua::LeiTianDaZhuang => "雷天大壯",
            LiuShiSiGua::HuoTianDaYou => "火天大有",
            LiuShiSiGua::ZeTianGuai => "澤天夬",
            LiuShiSiGua::QianWeiTian => "乾為天",
        };
        write!(f, "{}", name)
    }
}

impl LiuShiSiGua {
    /// 根据方图序号 (1-64) 获取六十四卦
    pub fn from_number(num: u8) -> Option<Self> {
        match num {
            // (1) 坤宮：內卦為坤
            1 => Some(Self::KunWeiDi),   // 2. 坤
            2 => Some(Self::ShanDiBo),   // 23. 剥
            3 => Some(Self::ShuiDiBi),   // 8. 比
            4 => Some(Self::FengDiGuan), // 20. 观
            5 => Some(Self::LeiDiYu),    // 16. 豫
            6 => Some(Self::HuoDiJin),   // 35. 晋
            7 => Some(Self::ZeDiCui),    // 45. 萃
            8 => Some(Self::TianDiPi),   // 12. 否

            // (9) 艮宮：內卦為艮
            9 => Some(Self::DiShanQian),      // 15. 谦
            10 => Some(Self::GenWeiShan),     // 52. 艮
            11 => Some(Self::ShuiShanJian),   // 39. 蹇
            12 => Some(Self::FengShanJian),   // 53. 渐
            13 => Some(Self::LeiShanXiaoGuo), // 62. 小过
            14 => Some(Self::HuoShanLv),      // 56. 旅
            15 => Some(Self::ZeShanXian),     // 31. 咸
            16 => Some(Self::TianShanDun),    // 33. 遯

            // (17) 坎宮：內卦為坎
            17 => Some(Self::DiShuiShi),    // 7. 师
            18 => Some(Self::ShanShuiMeng), // 4. 蒙
            19 => Some(Self::KanWeiShui),   // 29. 坎
            20 => Some(Self::FengShuiHuan), // 59. 涣
            21 => Some(Self::LeiShuiXie),   // 40. 解
            22 => Some(Self::HuoShuiWeiJi), // 64. 未济
            23 => Some(Self::ZeShuiKun),    // 47. 困
            24 => Some(Self::TianShuiSong), // 6. 讼

            // (25) 巽宮：內卦為巽
            25 => Some(Self::DiFengSheng),  // 46. 升
            26 => Some(Self::ShanFengGu),   // 18. 蛊
            27 => Some(Self::ShuiFengJing), // 48. 井
            28 => Some(Self::XunWeiFeng),   // 57. 巽
            29 => Some(Self::LeiFengHeng),  // 32. 恒
            30 => Some(Self::HuoFengDing),  // 50. 鼎
            31 => Some(Self::ZeFengDaGuo),  // 28. 大过
            32 => Some(Self::TianFengGou),  // 44. 姤

            // (33) 震宮：內卦為震
            33 => Some(Self::DiLeiFu),       // 24. 复
            34 => Some(Self::ShanLeiYi),     // 27. 颐
            35 => Some(Self::ShuiLeiTun),    // 3. 屯
            36 => Some(Self::FengLeiYi),     // 42. 益
            37 => Some(Self::ZhenWeiLei),    // 51. 震
            38 => Some(Self::HuoLeiShiHe),   // 21. 噬嗑
            39 => Some(Self::ZeLeiSui),      // 17. 随
            40 => Some(Self::TianLeiWuWang), // 25. 无妄

            // (41) 離宮：內卦為離
            41 => Some(Self::DiHuoMingYi),    // 36. 明夷
            42 => Some(Self::ShanHuoBi),      // 22. 贲
            43 => Some(Self::ShuiHuoJiJi),    // 63. 既济
            44 => Some(Self::FengHuoJiaRen),  // 37. 家人
            45 => Some(Self::LeiHuoFeng),     // 55. 丰
            46 => Some(Self::LiWeiHuo),       // 30. 离
            47 => Some(Self::ZeHuoGe),        // 49. 革
            48 => Some(Self::TianHuoTongRen), // 13. 同人

            // (49) 兌宮：內卦為兌
            49 => Some(Self::DiZeLin),       // 19. 临
            50 => Some(Self::ShanZeSun),     // 41. 损
            51 => Some(Self::ShuiZeJie),     // 60. 节
            52 => Some(Self::FengZeZhongFu), // 61. 中孚
            53 => Some(Self::LeiZeGuiMei),   // 54. 归妹
            54 => Some(Self::HuoZeKui),      // 38. 睽
            55 => Some(Self::DuiWeiZe),      // 58. 兑
            56 => Some(Self::TianZeLv),      // 10. 履

            // (57) 乾宮：內卦為乾
            57 => Some(Self::DiTianTai),       // 11. 泰
            58 => Some(Self::ShanTianDaXu),    // 26. 大畜
            59 => Some(Self::ShuiTianXu),      // 5. 需
            60 => Some(Self::FengTianXiaoXu),  // 9. 小畜
            61 => Some(Self::LeiTianDaZhuang), // 34. 大壮
            62 => Some(Self::HuoTianDaYou),    // 14. 大有
            63 => Some(Self::ZeTianGuai),      // 43. 夬
            64 => Some(Self::QianWeiTian),     // 1. 乾

            _ => None,
        }
    }

    /// 获取六十四卦对应的上下卦（重卦）
    ///
    /// 遵循伏羲先天方图逻辑：
    /// 内卦（下卦）：坤、艮、坎、巽、震、離、兌、乾（每8卦换一个内卦）
    /// 外卦（上卦）：坤、艮、坎、巽、震、離、兌、乾（每一组内循环一次）
    pub fn to_chong_gua(&self) -> ChongGua {
        match self {
            // --- 第一列：內卦為 坤 ---
            LiuShiSiGua::KunWeiDi => ChongGua::new(BaGua::Kun, BaGua::Kun),
            LiuShiSiGua::ShanDiBo => ChongGua::new(BaGua::Gen, BaGua::Kun),
            LiuShiSiGua::ShuiDiBi => ChongGua::new(BaGua::Kan, BaGua::Kun),
            LiuShiSiGua::FengDiGuan => ChongGua::new(BaGua::Xun, BaGua::Kun),
            LiuShiSiGua::LeiDiYu => ChongGua::new(BaGua::Zhen, BaGua::Kun),
            LiuShiSiGua::HuoDiJin => ChongGua::new(BaGua::Li, BaGua::Kun),
            LiuShiSiGua::ZeDiCui => ChongGua::new(BaGua::Dui, BaGua::Kun),
            LiuShiSiGua::TianDiPi => ChongGua::new(BaGua::Qian, BaGua::Kun),

            // --- 第二列：內卦為 艮 ---
            LiuShiSiGua::DiShanQian => ChongGua::new(BaGua::Kun, BaGua::Gen),
            LiuShiSiGua::GenWeiShan => ChongGua::new(BaGua::Gen, BaGua::Gen),
            LiuShiSiGua::ShuiShanJian => ChongGua::new(BaGua::Kan, BaGua::Gen),
            LiuShiSiGua::FengShanJian => ChongGua::new(BaGua::Xun, BaGua::Gen),
            LiuShiSiGua::LeiShanXiaoGuo => ChongGua::new(BaGua::Zhen, BaGua::Gen),
            LiuShiSiGua::HuoShanLv => ChongGua::new(BaGua::Li, BaGua::Gen),
            LiuShiSiGua::ZeShanXian => ChongGua::new(BaGua::Dui, BaGua::Gen),
            LiuShiSiGua::TianShanDun => ChongGua::new(BaGua::Qian, BaGua::Gen),

            // --- 第三列：內卦為 坎 ---
            LiuShiSiGua::DiShuiShi => ChongGua::new(BaGua::Kun, BaGua::Kan),
            LiuShiSiGua::ShanShuiMeng => ChongGua::new(BaGua::Gen, BaGua::Kan),
            LiuShiSiGua::KanWeiShui => ChongGua::new(BaGua::Kan, BaGua::Kan),
            LiuShiSiGua::FengShuiHuan => ChongGua::new(BaGua::Xun, BaGua::Kan),
            LiuShiSiGua::LeiShuiXie => ChongGua::new(BaGua::Zhen, BaGua::Kan),
            LiuShiSiGua::HuoShuiWeiJi => ChongGua::new(BaGua::Li, BaGua::Kan),
            LiuShiSiGua::ZeShuiKun => ChongGua::new(BaGua::Dui, BaGua::Kan),
            LiuShiSiGua::TianShuiSong => ChongGua::new(BaGua::Qian, BaGua::Kan),

            // --- 第四列：內卦為 巽 ---
            LiuShiSiGua::DiFengSheng => ChongGua::new(BaGua::Kun, BaGua::Xun),
            LiuShiSiGua::ShanFengGu => ChongGua::new(BaGua::Gen, BaGua::Xun),
            LiuShiSiGua::ShuiFengJing => ChongGua::new(BaGua::Kan, BaGua::Xun),
            LiuShiSiGua::XunWeiFeng => ChongGua::new(BaGua::Xun, BaGua::Xun),
            LiuShiSiGua::LeiFengHeng => ChongGua::new(BaGua::Zhen, BaGua::Xun),
            LiuShiSiGua::HuoFengDing => ChongGua::new(BaGua::Li, BaGua::Xun),
            LiuShiSiGua::ZeFengDaGuo => ChongGua::new(BaGua::Dui, BaGua::Xun),
            LiuShiSiGua::TianFengGou => ChongGua::new(BaGua::Qian, BaGua::Xun),

            // --- 第五列：內卦為 震 ---
            LiuShiSiGua::DiLeiFu => ChongGua::new(BaGua::Kun, BaGua::Zhen),
            LiuShiSiGua::ShanLeiYi => ChongGua::new(BaGua::Gen, BaGua::Zhen),
            LiuShiSiGua::ShuiLeiTun => ChongGua::new(BaGua::Kan, BaGua::Zhen),
            LiuShiSiGua::FengLeiYi => ChongGua::new(BaGua::Xun, BaGua::Zhen),
            LiuShiSiGua::ZhenWeiLei => ChongGua::new(BaGua::Zhen, BaGua::Zhen),
            LiuShiSiGua::HuoLeiShiHe => ChongGua::new(BaGua::Li, BaGua::Zhen),
            LiuShiSiGua::ZeLeiSui => ChongGua::new(BaGua::Dui, BaGua::Zhen),
            LiuShiSiGua::TianLeiWuWang => ChongGua::new(BaGua::Qian, BaGua::Zhen),

            // --- 第六列：內卦為 離 ---
            LiuShiSiGua::DiHuoMingYi => ChongGua::new(BaGua::Kun, BaGua::Li),
            LiuShiSiGua::ShanHuoBi => ChongGua::new(BaGua::Gen, BaGua::Li),
            LiuShiSiGua::ShuiHuoJiJi => ChongGua::new(BaGua::Kan, BaGua::Li),
            LiuShiSiGua::FengHuoJiaRen => ChongGua::new(BaGua::Xun, BaGua::Li),
            LiuShiSiGua::LeiHuoFeng => ChongGua::new(BaGua::Zhen, BaGua::Li),
            LiuShiSiGua::LiWeiHuo => ChongGua::new(BaGua::Li, BaGua::Li),
            LiuShiSiGua::ZeHuoGe => ChongGua::new(BaGua::Dui, BaGua::Li),
            LiuShiSiGua::TianHuoTongRen => ChongGua::new(BaGua::Qian, BaGua::Li),

            // --- 第七列：內卦為 兌 ---
            LiuShiSiGua::DiZeLin => ChongGua::new(BaGua::Kun, BaGua::Dui),
            LiuShiSiGua::ShanZeSun => ChongGua::new(BaGua::Gen, BaGua::Dui),
            LiuShiSiGua::ShuiZeJie => ChongGua::new(BaGua::Kan, BaGua::Dui),
            LiuShiSiGua::FengZeZhongFu => ChongGua::new(BaGua::Xun, BaGua::Dui),
            LiuShiSiGua::LeiZeGuiMei => ChongGua::new(BaGua::Zhen, BaGua::Dui),
            LiuShiSiGua::HuoZeKui => ChongGua::new(BaGua::Li, BaGua::Dui),
            LiuShiSiGua::DuiWeiZe => ChongGua::new(BaGua::Dui, BaGua::Dui),
            LiuShiSiGua::TianZeLv => ChongGua::new(BaGua::Qian, BaGua::Dui),

            // --- 第八列：內卦為 乾 ---
            LiuShiSiGua::DiTianTai => ChongGua::new(BaGua::Kun, BaGua::Qian),
            LiuShiSiGua::ShanTianDaXu => ChongGua::new(BaGua::Gen, BaGua::Qian),
            LiuShiSiGua::ShuiTianXu => ChongGua::new(BaGua::Kan, BaGua::Qian),
            LiuShiSiGua::FengTianXiaoXu => ChongGua::new(BaGua::Xun, BaGua::Qian),
            LiuShiSiGua::LeiTianDaZhuang => ChongGua::new(BaGua::Zhen, BaGua::Qian),
            LiuShiSiGua::HuoTianDaYou => ChongGua::new(BaGua::Li, BaGua::Qian),
            LiuShiSiGua::ZeTianGuai => ChongGua::new(BaGua::Dui, BaGua::Qian),
            LiuShiSiGua::QianWeiTian => ChongGua::new(BaGua::Qian, BaGua::Qian),
        }
    }

    pub fn chong_gua_compact(num: u8) -> Option<ChongGua> {
        if num < 1 || num > 64 {
            return None;
        }

        // 伏羲方图序列：坤8, 艮7, 坎6, 巽5, 震4, 离3, 兑2, 乾1 (对应您的 BaGua::from_number 逻辑)
        // 注意：您代码里的 BaGua::from_number(1) 是 Qian，(8) 是 Kun
        // 方图每列内卦顺序：坤(8)->艮(7)->坎(6)->巽(5)->震(4)->离(3)->兑(2)->乾(1)

        let inner_idx = 8 - ((num - 1) / 8); // 计算内卦（列）
        let outer_idx = 8 - ((num - 1) % 8); // 计算外卦（行）

        Some(ChongGua::new(
            BaGua::from_number(outer_idx as u32)?,
            BaGua::from_number(inner_idx as u32)?,
        ))
    }

    /// 获取六十四卦对应的 Unicode 符号 (\u{4DC0} - \u{4DFF})
    /// 注意：Unicode 标准采用的是《周易》卦序
    pub fn unicode_symbol(&self) -> char {
        match self {
            // 第 1 列 (內坤)
            LiuShiSiGua::KunWeiDi => '\u{4DC1}',   // 02. 坤
            LiuShiSiGua::ShanDiBo => '\u{4DD6}',   // 23. 剝
            LiuShiSiGua::ShuiDiBi => '\u{4DC7}',   // 08. 比
            LiuShiSiGua::FengDiGuan => '\u{4DD3}', // 20. 觀
            LiuShiSiGua::LeiDiYu => '\u{4DCF}',    // 16. 豫
            LiuShiSiGua::HuoDiJin => '\u{4DE2}',   // 35. 晉
            LiuShiSiGua::ZeDiCui => '\u{4DEC}',    // 45. 萃
            LiuShiSiGua::TianDiPi => '\u{4DCB}',   // 12. 否

            // 第 2 列 (內艮)
            LiuShiSiGua::DiShanQian => '\u{4DCE}',   // 15. 謙
            LiuShiSiGua::GenWeiShan => '\u{4DF3}',   // 52. 艮
            LiuShiSiGua::ShuiShanJian => '\u{4DE6}', // 39. 蹇
            LiuShiSiGua::FengShanJian => '\u{4DF4}', // 53. 漸
            LiuShiSiGua::LeiShanXiaoGuo => '\u{4DFD}', // 62. 小過
            LiuShiSiGua::HuoShanLv => '\u{4DF7}',    // 56. 旅
            LiuShiSiGua::ZeShanXian => '\u{4DDE}',   // 31. 咸
            LiuShiSiGua::TianShanDun => '\u{4DE0}',  // 33. 遯

            // 第 3 列 (內坎)
            LiuShiSiGua::DiShuiShi => '\u{4DC6}',    // 07. 師
            LiuShiSiGua::ShanShuiMeng => '\u{4DC3}', // 04. 蒙
            LiuShiSiGua::KanWeiShui => '\u{4DDC}',   // 29. 坎
            LiuShiSiGua::FengShuiHuan => '\u{4DFA}', // 59. 渙
            LiuShiSiGua::LeiShuiXie => '\u{4DE7}',   // 40. 解
            LiuShiSiGua::HuoShuiWeiJi => '\u{4DFF}', // 64. 未濟
            LiuShiSiGua::ZeShuiKun => '\u{4DEE}',    // 47. 困
            LiuShiSiGua::TianShuiSong => '\u{4DC5}', // 06. 訟

            // 第 4 列 (內巽)
            LiuShiSiGua::DiFengSheng => '\u{4DED}',  // 46. 升
            LiuShiSiGua::ShanFengGu => '\u{4DD1}',   // 18. 蠱
            LiuShiSiGua::ShuiFengJing => '\u{4DEF}', // 48. 井
            LiuShiSiGua::XunWeiFeng => '\u{4DF8}',   // 57. 巽
            LiuShiSiGua::LeiFengHeng => '\u{4DDF}',  // 32. 恆
            LiuShiSiGua::HuoFengDing => '\u{4DF1}',  // 50. 鼎
            LiuShiSiGua::ZeFengDaGuo => '\u{4DDB}',  // 28. 大過
            LiuShiSiGua::TianFengGou => '\u{4DEB}',  // 44. 姤

            // 第 5 列 (內震)
            LiuShiSiGua::DiLeiFu => '\u{4DD7}',       // 24. 復
            LiuShiSiGua::ShanLeiYi => '\u{4DDA}',     // 27. 頤
            LiuShiSiGua::ShuiLeiTun => '\u{4DC2}',    // 03. 屯
            LiuShiSiGua::FengLeiYi => '\u{4DE9}',     // 42. 益
            LiuShiSiGua::ZhenWeiLei => '\u{4DF2}',    // 51. 震
            LiuShiSiGua::HuoLeiShiHe => '\u{4DD4}',   // 21. 噬嗑
            LiuShiSiGua::ZeLeiSui => '\u{4DD0}',      // 17. 隨
            LiuShiSiGua::TianLeiWuWang => '\u{4DD8}', // 25. 无妄

            // 第 6 列 (內離)
            LiuShiSiGua::DiHuoMingYi => '\u{4DE3}', // 36. 明夷
            LiuShiSiGua::ShanHuoBi => '\u{4DD5}',   // 22. 賁
            LiuShiSiGua::ShuiHuoJiJi => '\u{4DFE}', // 63. 既濟
            LiuShiSiGua::FengHuoJiaRen => '\u{4DE4}', // 37. 家人
            LiuShiSiGua::LeiHuoFeng => '\u{4DF6}',  // 55. 豐
            LiuShiSiGua::LiWeiHuo => '\u{4DDD}',    // 30. 離
            LiuShiSiGua::ZeHuoGe => '\u{4DF0}',     // 49. 革
            LiuShiSiGua::TianHuoTongRen => '\u{4DCC}', // 13. 同人

            // 第 7 列 (內兌)
            LiuShiSiGua::DiZeLin => '\u{4DD2}',       // 19. 臨
            LiuShiSiGua::ShanZeSun => '\u{4DE8}',     // 41. 損
            LiuShiSiGua::ShuiZeJie => '\u{4DFB}',     // 60. 節
            LiuShiSiGua::FengZeZhongFu => '\u{4DFC}', // 61. 中孚
            LiuShiSiGua::LeiZeGuiMei => '\u{4DF5}',   // 54. 歸妹
            LiuShiSiGua::HuoZeKui => '\u{4DE5}',      // 38. 睽
            LiuShiSiGua::DuiWeiZe => '\u{4DF9}',      // 58. 兌
            LiuShiSiGua::TianZeLv => '\u{4DC9}',      // 10. 履

            // 第 8 列 (內乾)
            LiuShiSiGua::DiTianTai => '\u{4DCA}',      // 11. 泰
            LiuShiSiGua::ShanTianDaXu => '\u{4DD9}',   // 26. 大畜
            LiuShiSiGua::ShuiTianXu => '\u{4DC4}',     // 05. 需
            LiuShiSiGua::FengTianXiaoXu => '\u{4DC8}', // 09. 小畜
            LiuShiSiGua::LeiTianDaZhuang => '\u{4DE1}', // 34. 大壯
            LiuShiSiGua::HuoTianDaYou => '\u{4DCD}',   // 14. 大有
            LiuShiSiGua::ZeTianGuai => '\u{4DEA}',     // 43. 夬
            LiuShiSiGua::QianWeiTian => '\u{4DC0}',    // 01. 乾
        }
    }
}

impl ChongGua {
    /// 将重卦组合转换回 LiuShiSiGua 枚举
    /// 逻辑：根据内外卦（上下卦）的组合，定位其在方图中的位置
    pub fn to_liu_shi_si_gua(&self) -> LiuShiSiGua {
        match (self.shang, self.xia) {
            // --- 内卦为坤 (第1列) ---
            (BaGua::Kun, BaGua::Kun) => LiuShiSiGua::KunWeiDi,
            (BaGua::Gen, BaGua::Kun) => LiuShiSiGua::ShanDiBo,
            (BaGua::Kan, BaGua::Kun) => LiuShiSiGua::ShuiDiBi,
            (BaGua::Xun, BaGua::Kun) => LiuShiSiGua::FengDiGuan,
            (BaGua::Zhen, BaGua::Kun) => LiuShiSiGua::LeiDiYu,
            (BaGua::Li, BaGua::Kun) => LiuShiSiGua::HuoDiJin,
            (BaGua::Dui, BaGua::Kun) => LiuShiSiGua::ZeDiCui,
            (BaGua::Qian, BaGua::Kun) => LiuShiSiGua::TianDiPi,

            // --- 内卦为艮 (第2列) ---
            (BaGua::Kun, BaGua::Gen) => LiuShiSiGua::DiShanQian,
            (BaGua::Gen, BaGua::Gen) => LiuShiSiGua::GenWeiShan,
            (BaGua::Kan, BaGua::Gen) => LiuShiSiGua::ShuiShanJian,
            (BaGua::Xun, BaGua::Gen) => LiuShiSiGua::FengShanJian,
            (BaGua::Zhen, BaGua::Gen) => LiuShiSiGua::LeiShanXiaoGuo,
            (BaGua::Li, BaGua::Gen) => LiuShiSiGua::HuoShanLv,
            (BaGua::Dui, BaGua::Gen) => LiuShiSiGua::ZeShanXian,
            (BaGua::Qian, BaGua::Gen) => LiuShiSiGua::TianShanDun,

            // --- 内卦为坎 (第3列) ---
            (BaGua::Kun, BaGua::Kan) => LiuShiSiGua::DiShuiShi,
            (BaGua::Gen, BaGua::Kan) => LiuShiSiGua::ShanShuiMeng,
            (BaGua::Kan, BaGua::Kan) => LiuShiSiGua::KanWeiShui,
            (BaGua::Xun, BaGua::Kan) => LiuShiSiGua::FengShuiHuan,
            (BaGua::Zhen, BaGua::Kan) => LiuShiSiGua::LeiShuiXie,
            (BaGua::Li, BaGua::Kan) => LiuShiSiGua::HuoShuiWeiJi,
            (BaGua::Dui, BaGua::Kan) => LiuShiSiGua::ZeShuiKun,
            (BaGua::Qian, BaGua::Kan) => LiuShiSiGua::TianShuiSong,

            // --- 内卦为巽 (第4列) ---
            (BaGua::Kun, BaGua::Xun) => LiuShiSiGua::DiFengSheng,
            (BaGua::Gen, BaGua::Xun) => LiuShiSiGua::ShanFengGu,
            (BaGua::Kan, BaGua::Xun) => LiuShiSiGua::ShuiFengJing,
            (BaGua::Xun, BaGua::Xun) => LiuShiSiGua::XunWeiFeng,
            (BaGua::Zhen, BaGua::Xun) => LiuShiSiGua::LeiFengHeng,
            (BaGua::Li, BaGua::Xun) => LiuShiSiGua::HuoFengDing,
            (BaGua::Dui, BaGua::Xun) => LiuShiSiGua::ZeFengDaGuo,
            (BaGua::Qian, BaGua::Xun) => LiuShiSiGua::TianFengGou,

            // --- 内卦为震 (第5列) ---
            (BaGua::Kun, BaGua::Zhen) => LiuShiSiGua::DiLeiFu,
            (BaGua::Gen, BaGua::Zhen) => LiuShiSiGua::ShanLeiYi,
            (BaGua::Kan, BaGua::Zhen) => LiuShiSiGua::ShuiLeiTun,
            (BaGua::Xun, BaGua::Zhen) => LiuShiSiGua::FengLeiYi,
            (BaGua::Zhen, BaGua::Zhen) => LiuShiSiGua::ZhenWeiLei,
            (BaGua::Li, BaGua::Zhen) => LiuShiSiGua::HuoLeiShiHe,
            (BaGua::Dui, BaGua::Zhen) => LiuShiSiGua::ZeLeiSui,
            (BaGua::Qian, BaGua::Zhen) => LiuShiSiGua::TianLeiWuWang,

            // --- 内卦为离 (第6列) ---
            (BaGua::Kun, BaGua::Li) => LiuShiSiGua::DiHuoMingYi,
            (BaGua::Gen, BaGua::Li) => LiuShiSiGua::ShanHuoBi,
            (BaGua::Kan, BaGua::Li) => LiuShiSiGua::ShuiHuoJiJi,
            (BaGua::Xun, BaGua::Li) => LiuShiSiGua::FengHuoJiaRen,
            (BaGua::Zhen, BaGua::Li) => LiuShiSiGua::LeiHuoFeng,
            (BaGua::Li, BaGua::Li) => LiuShiSiGua::LiWeiHuo,
            (BaGua::Dui, BaGua::Li) => LiuShiSiGua::ZeHuoGe,
            (BaGua::Qian, BaGua::Li) => LiuShiSiGua::TianHuoTongRen,

            // --- 内卦为兑 (第7列) ---
            (BaGua::Kun, BaGua::Dui) => LiuShiSiGua::DiZeLin,
            (BaGua::Gen, BaGua::Dui) => LiuShiSiGua::ShanZeSun,
            (BaGua::Kan, BaGua::Dui) => LiuShiSiGua::ShuiZeJie,
            (BaGua::Xun, BaGua::Dui) => LiuShiSiGua::FengZeZhongFu,
            (BaGua::Zhen, BaGua::Dui) => LiuShiSiGua::LeiZeGuiMei,
            (BaGua::Li, BaGua::Dui) => LiuShiSiGua::HuoZeKui,
            (BaGua::Dui, BaGua::Dui) => LiuShiSiGua::DuiWeiZe,
            (BaGua::Qian, BaGua::Dui) => LiuShiSiGua::TianZeLv,

            // --- 内卦为乾 (第8列) ---
            (BaGua::Kun, BaGua::Qian) => LiuShiSiGua::DiTianTai,
            (BaGua::Gen, BaGua::Qian) => LiuShiSiGua::ShanTianDaXu,
            (BaGua::Kan, BaGua::Qian) => LiuShiSiGua::ShuiTianXu,
            (BaGua::Xun, BaGua::Qian) => LiuShiSiGua::FengTianXiaoXu,
            (BaGua::Zhen, BaGua::Qian) => LiuShiSiGua::LeiTianDaZhuang,
            (BaGua::Li, BaGua::Qian) => LiuShiSiGua::HuoTianDaYou,
            (BaGua::Dui, BaGua::Qian) => LiuShiSiGua::ZeTianGuai,
            (BaGua::Qian, BaGua::Qian) => LiuShiSiGua::QianWeiTian,
        }
    }

    /// 获取变卦（之卦）
    pub fn bian_gua(&self, dong_yao_index: u8) -> Option<Self> {
        if !(1..=6).contains(&dong_yao_index) {
            return None; // 动爻只能是 1 到 6
        }

        let mut yaos = self.yao_xiang();
        let idx = (dong_yao_index - 1) as usize;

        yaos[idx] = match yaos[idx] {
            Yao::Yang => Yao::Yin,
            Yao::Yin => Yao::Yang,
        };

        let xia = BaGua::from_yao_xiang([yaos[0], yaos[1], yaos[2]]);
        let shang = BaGua::from_yao_xiang([yaos[3], yaos[4], yaos[5]]);

        Some(ChongGua::new(shang, xia))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gua_sequence_integrity() {
        // 遍历所有 1-64 序号
        for i in 1..=64 {
            let gua = LiuShiSiGua::from_number(i).expect("序号应在1-64之间");
            let chong_gua = gua.to_chong_gua();
            let back_to_gua = chong_gua.to_liu_shi_si_gua();

            // 验证转换后是否还是原来的卦
            assert_eq!(gua, back_to_gua, "序号 {} 的转换逻辑不匹配", i);
        }
    }

    #[test]
    fn test_specific_gua_logic() {
        // 抽查：乾为天 (序号 64, 因为你的方图乾宫在最后)
        let qian = LiuShiSiGua::from_number(64).unwrap();
        assert_eq!(qian, LiuShiSiGua::QianWeiTian);
        assert_eq!(qian.to_chong_gua().shang, BaGua::Qian);
        assert_eq!(qian.to_chong_gua().xia, BaGua::Qian);

        // 抽查：火雷噬嗑 (序号 38)
        let shi_he = LiuShiSiGua::from_number(38).unwrap();
        assert_eq!(shi_he.to_chong_gua().shang, BaGua::Li); // 火
        assert_eq!(shi_he.to_chong_gua().xia, BaGua::Zhen); // 雷
    }

    #[test]
    fn test_bian_gua() {
        // 以 乾卦 为例，初爻动
        let qian = ChongGua::new(BaGua::Qian, BaGua::Qian);
        let bian = qian.bian_gua(1).unwrap(); // 初爻在下卦

        // 乾卦初爻（阳）变阴 -> 下卦变为 巽 (巽为 [阴, 阳, 阳]) -> 实际上你的 BaGua::Xun 是 [Yin, Yang, Yang]
        // 注意：根据你的 from_yao_xiang，[Yin, Yang, Yang] 是巽，[Yang, Yin, Yin] 是震
        // 乾 [Yang, Yang, Yang] 初爻变后为 [Yin, Yang, Yang] 即 巽
        assert_eq!(bian.xia, BaGua::Xun);
        assert_eq!(bian.shang, BaGua::Qian); // 上卦不动

        // 验证变卦后的卦名是否正确 (天风姤)
        assert_eq!(bian.to_liu_shi_si_gua(), LiuShiSiGua::TianFengGou);
    }

    #[test]
    fn test_metadata() {
        // 验证八卦符号
        assert_eq!(BaGua::Qian.unicode_symbol(), '☰');
        assert_eq!(BaGua::Kun.unicode_symbol(), '☷');

        // 验证六十四卦符号
        assert_eq!(LiuShiSiGua::QianWeiTian.unicode_symbol(), '䷀');
        assert_eq!(LiuShiSiGua::KunWeiDi.unicode_symbol(), '䷁');

        // 验证五行
        assert_eq!(BaGua::Li.wu_xing().to_string(), "火");
    }
    #[test]
    fn test_ti_yong() {
        let gua = ChongGua::new(BaGua::Li, BaGua::Zhen); // 火雷噬嗑
        // 动爻 1 (在下卦震) -> 下卦为用，上卦为体
        let (ti, yong) = gua.ti_yong(1).unwrap();
        assert_eq!(ti, BaGua::Li);
        assert_eq!(yong, BaGua::Zhen);

        // 动爻 4 (在上卦离) -> 上卦为用，下卦为体
        let (ti2, yong2) = gua.ti_yong(4).unwrap();
        assert_eq!(ti2, BaGua::Zhen);
        assert_eq!(yong2, BaGua::Li);
    }

    #[test]
    fn test_bagua_from_number_boundary() {
        // 测试有效边界值
        assert_eq!(BaGua::from_number(1), Some(BaGua::Qian));
        assert_eq!(BaGua::from_number(8), Some(BaGua::Kun));
        
        // 测试无效边界值
        assert_eq!(BaGua::from_number(0), None);
        assert_eq!(BaGua::from_number(9), None);
        assert_eq!(BaGua::from_number(u32::MAX), None);
    }

    #[test]
    fn test_bagua_yao_xiang() {
        // 测试乾卦
        let qian = BaGua::Qian;
        let yaoxiang_qian = qian.yao_xiang();
        assert_eq!(yaoxiang_qian, [Yao::Yang, Yao::Yang, Yao::Yang]);

        // 测试兑卦
        let dui = BaGua::Dui;
        let yaoxiang_dui = dui.yao_xiang();
        assert_eq!(yaoxiang_dui, [Yao::Yang, Yao::Yang, Yao::Yin]);

        // 测试离卦
        let li = BaGua::Li;
        let yaoxiang_li = li.yao_xiang();
        assert_eq!(yaoxiang_li, [Yao::Yang, Yao::Yin, Yao::Yang]);

        // 测试震卦
        let zhen = BaGua::Zhen;
        let yaoxiang_zhen = zhen.yao_xiang();
        assert_eq!(yaoxiang_zhen, [Yao::Yang, Yao::Yin, Yao::Yin]);

        // 测试巽卦
        let xun = BaGua::Xun;
        let yaoxiang_xun = xun.yao_xiang();
        assert_eq!(yaoxiang_xun, [Yao::Yin, Yao::Yang, Yao::Yang]);

        // 测试坎卦
        let kan = BaGua::Kan;
        let yaoxiang_kan = kan.yao_xiang();
        assert_eq!(yaoxiang_kan, [Yao::Yin, Yao::Yang, Yao::Yin]);

        // 测试艮卦
        let gen_gua = BaGua::Gen;
        let yaoxiang_gen = gen_gua.yao_xiang();
        assert_eq!(yaoxiang_gen, [Yao::Yin, Yao::Yin, Yao::Yang]);

        // 测试坤卦
        let kun = BaGua::Kun;
        let yaoxiang_kun = kun.yao_xiang();
        assert_eq!(yaoxiang_kun, [Yao::Yin, Yao::Yin, Yao::Yin]);
    }

    #[test]
    fn test_bagua_from_yao_xiang() {
        // 测试乾卦
        let qian = BaGua::from_yao_xiang([Yao::Yang, Yao::Yang, Yao::Yang]);
        assert_eq!(qian, BaGua::Qian);

        // 测试兑卦
        let dui = BaGua::from_yao_xiang([Yao::Yang, Yao::Yang, Yao::Yin]);
        assert_eq!(dui, BaGua::Dui);

        // 测试离卦
        let li = BaGua::from_yao_xiang([Yao::Yang, Yao::Yin, Yao::Yang]);
        assert_eq!(li, BaGua::Li);

        // 测试震卦
        let zhen = BaGua::from_yao_xiang([Yao::Yang, Yao::Yin, Yao::Yin]);
        assert_eq!(zhen, BaGua::Zhen);

        // 测试巽卦
        let xun = BaGua::from_yao_xiang([Yao::Yin, Yao::Yang, Yao::Yang]);
        assert_eq!(xun, BaGua::Xun);

        // 测试坎卦
        let kan = BaGua::from_yao_xiang([Yao::Yin, Yao::Yang, Yao::Yin]);
        assert_eq!(kan, BaGua::Kan);

        // 测试艮卦
        let gen_gua = BaGua::from_yao_xiang([Yao::Yin, Yao::Yin, Yao::Yang]);
        assert_eq!(gen_gua, BaGua::Gen);

        // 测试坤卦
        let kun = BaGua::from_yao_xiang([Yao::Yin, Yao::Yin, Yao::Yin]);
        assert_eq!(kun, BaGua::Kun);
    }
}
