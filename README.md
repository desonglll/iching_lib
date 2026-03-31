# 易经占卜库 (I Ching Divination Library)

一个用 Rust 实现的易经占卜库，提供完整的起卦、断卦功能。

## 功能特性

- **八卦 (Eight Trigrams)**: 支持乾、兑、离、震、巽、坎、艮、坤八卦
  - 先天数与后天九宫数
  - 五行属性
  - 阴阳爻象
  - Unicode 符号显示

- **六十四卦 (Sixty-Four Hexagrams)**: 完整的六十四卦系统
  - 伏羲先天方图排列
  - 互卦、错卦、综卦变换
  - Unicode 符号支持

- **体用断卦**: 传统梅花易数断卦法
  - 体卦与用卦分析
  - 五行生克关系判断
  - 动爻计算

- **五行系统 (Five Elements)**: 金木水火土五行
  - 五行生克关系
  - 季节旺相休囚死状态

- **时空起卦**: 支持多种起卦方式
  - 数字起卦
  - 时间起卦（含时辰计算）
  - 专业节气季节判断

## 项目结构

```
src/
├── main.rs      # 主程序入口，演示完整的占卜流程
├── lib.rs       # 库文件模块声明
├── gua.rs       # 八卦与六十四卦定义
├── yao.rs       # 爻（阴阳）定义与动爻计算
├── wu_xing.rs   # 五行系统
├── core.rs      # 核心测试
└── shi_chen.rs  # 时辰与季节计算
```

## 依赖项

- [chrono](https://crates.io/crates/chrono): 日期时间处理
- [lunar_rust](https://crates.io/crates/lunar_rust): 农历与八字计算

## 安装

确保已安装 Rust 工具链，然后克隆项目并运行：

```bash
git clone <repository-url>
cd i_ching
cargo build --release
cargo run
```

## 使用示例

### 基本占卜流程

```rust
use iching_lib::gua::{BaGua, ChongGua};
use iching_lib::yao::get_dong_yao;

// 1. 根据数字起卦
let shang = BaGua::from_number(1).unwrap(); // 乾卦
let xia = BaGua::from_number(8).unwrap();   // 坤卦

// 2. 组成本卦
let ben_gua = ChongGua::new(shang, xia); // 天地否

// 3. 计算动爻
let dong_yao = get_dong_yao(1, 8, Some(4));

// 4. 分析体用
let (ti, yong) = ben_gua.ti_yong(dong_yao).unwrap();

// 5. 获取变卦
let bian_gua = ben_gua.bian_gua(dong_yao).unwrap();

// 6. 获取互卦
let hu_gua = ben_gua.hu_gua();
```

### 五行生克分析

```rust
use iching_lib::wu_xing::WuXing;

let jin = WuXing::Jin;
let mu = WuXing::Mu;

// 判断五行关系
let relation = jin.relation_with(mu);
println!("{}", relation.description()); // 输出：【体克用】小吉...
```

### 获取当前时辰与季节

```rust
use iching_lib::shi_chen::{get_current_shichen_num, get_current_season_pro};

let shichen = get_current_shichen_num(); // 1-12 (子亥)
let season = get_current_season_pro();   // 1-5 (春夏秋四季末)
```

## 输出示例

运行 `cargo run` 将输出完整的占卜分析：

```
挑选的两个数字是：1, 1
---
--- 时空起卦 ---
当前时间：2025-03-31 12:00:00
当前季节：2 | 当前时辰数：4

上卦：乾    ☰    金
下卦：乾    ☰    金
---
本卦：
上卦（外卦）：乾
下卦（内卦）：乾
六十四卦：	䷀	乾為天
---
动爻：2
---
体：乾，用：乾
---
变卦：...
互卦（过程）：...
---
【本卦 - 现状】：【体用比和】中吉。上下同欲，伙伴得力，环境稳定。
【互卦 - 过程】：体卦受...影响，受...影响
【变卦 - 结果】：...
```

## 核心概念

### 梅花易数断卦逻辑

1. **体卦**: 代表自己、主体（无动爻的单卦）
2. **用卦**: 代表事件、客体（有动爻的单卦）
3. **互卦**: 过程、发展趋势
4. **变卦**: 最终结果
5. **错卦**: 事物的对立面
6. **综卦**: 换位思考的视角

### 五行生克关系

| 关系 | 说明 | 吉凶 |
|------|------|------|
| 用生体 | 外部助力 | 大吉 |
| 体生用 | 自身消耗 | 小凶 |
| 用克体 | 外部压力 | 大凶 |
| 体克用 | 劳而有获 | 小吉 |
| 比和 | 同等助力 | 中吉 |

## 测试

运行测试确保功能正常：

```bash
cargo test
```

## 许可证

本项目采用 MIT 许可证。

## 参考

- 《周易》
- 《梅花易数》
- 伏羲先天六十四卦方图
- 洛书后天八卦

## 贡献

欢迎提交 Issue 和 Pull Request！