use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WuXing {
    Jin,
    Mu,
    Shui,
    Huo,
    Tu,
}

impl Display for WuXing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WuXing::Jin => f.write_str("金"),
            WuXing::Mu => f.write_str("木"),
            WuXing::Shui => f.write_str("水"),
            WuXing::Huo => f.write_str("火"),
            WuXing::Tu => f.write_str("土"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Relation {
    ShengWo, // 生我 (大吉 - 生气)
    WoSheng, // 我生 (小凶 - 泄气)
    KeWo,    // 克我 (大凶 - 煞气)
    WoKe,    // 我克 (小吉 - 财气)
    BiHe,    // 比和 (中吉 - 帮扶)
}

impl Relation {
    pub fn description(&self) -> &'static str {
        match self {
            Relation::ShengWo => "【用生体】大吉。得外部助力，事半功倍，贵人相助。",
            Relation::WoSheng => "【体生用】小凶。自身能量泄气，操劳过度，多有耗损。",
            Relation::KeWo => "【用克体】大凶。压力极大，受人掣肘，恐有官非或疾厄。",
            Relation::WoKe => "【体克用】小吉。劳而有获，可以求财，但需费一番周折。",
            Relation::BiHe => "【体用比和】中吉。上下同欲，伙伴得力，环境稳定。",
        }
    }
}

impl WuXing {
    /// 判断当前五行(self)与对方五行(other)的关系
    pub fn relation_with(&self, other: WuXing) -> Relation {
        use WuXing::*;
        match (self, other) {
            // 比和：属性相同
            (s, o) if s == &o => Relation::BiHe,

            // 生我
            (Jin, Tu) | (Mu, Shui) | (Shui, Jin) | (Huo, Mu) | (Tu, Huo) => Relation::ShengWo,

            // 我生
            (Jin, Shui) | (Mu, Huo) | (Shui, Mu) | (Huo, Tu) | (Tu, Jin) => Relation::WoSheng,

            // 克我
            (Jin, Huo) | (Mu, Jin) | (Shui, Tu) | (Huo, Shui) | (Tu, Mu) => Relation::KeWo,

            // 我克
            (Jin, Mu) | (Mu, Tu) | (Shui, Huo) | (Huo, Jin) | (Tu, Shui) => Relation::WoKe,

            _ => unreachable!(),
        }
    }

    /// 判断当前五行在当前季节的能量状态
    pub fn get_energy_status(&self, season: u8) -> &'static str {
        use WuXing::*;
        match (season, self) {
            (1, Mu) => "旺（最强）",
            (1, Huo) => "相（次强）",
            (1, Shui) => "休",
            (1, Tu) => "囚",
            (1, Jin) => "死",
            (2, Huo) => "旺（最强）",
            (2, Tu) => "相（次强）",
            (2, Mu) => "休",
            (2, Shui) => "囚",
            (2, Jin) => "死",
            (3, Jin) => "旺（最强）",
            (3, Shui) => "相（次强）",
            (3, Tu) => "休",
            (3, Huo) => "囚",
            (3, Mu) => "死",
            (4, Shui) => "旺（最强）",
            (4, Mu) => "相（次强）",
            (4, Jin) => "休",
            (4, Tu) => "囚",
            (4, Huo) => "死",
            _ => "平（土旺）",
        }
    }
}
