use chrono::Local;
use i_ching::gua::{BaGua, ChongGua};
use i_ching::shi_chen::{get_current_season, get_current_shichen_num};
use i_ching::yao::get_dong_yao;

fn main() {

    let num1 = 1;
    let num2 = 1;
    println!("挑选的两个数字是：{num1}， {num2}");
    println!("---");

    let shichen_num = 4;
    let season = get_current_season();
    let now = Local::now();

    println!("--- 时空起卦 ---");
    println!("当前时间：{}", now.format("%Y-%m-%d %H:%M:%S"));
    println!("当前季节：{} | 当前时辰数：{}", season, shichen_num);

    let shang = BaGua::from_number(num1).unwrap();
    println!("上卦：");
    print!("{}", shang);
    print!("\t{}\t",shang.unicode_symbol());
    println!("{}", shang.wu_xing());
    let xia = BaGua::from_number(num2).unwrap();
    println!("下卦：");
    print!("{}", xia);
    print!("\t{}\t", xia.unicode_symbol());
    println!("{}", xia.wu_xing());
    println!("---");

    let ben_gua = ChongGua::new(shang, xia);
    println!("本卦：");
    println!("{}", ben_gua);
    print!("六十四卦：\t{}\t", ben_gua.to_liu_shi_si_gua().unicode_symbol());
    println!("{}", ben_gua.to_liu_shi_si_gua());
    println!("---");

    let dong_yao = get_dong_yao(num1 as u8, num2 as u8, Some(shichen_num));
    println!("动爻：{dong_yao}");
    println!("---");

    let (ti, yong) = ben_gua.ti_yong(dong_yao).unwrap();
    println!("体：{ti}，用：{yong}");
    println!("---");

    let bian_gua = ben_gua.bian_gua(dong_yao);
    println!("变卦：{bian_gua}");
    print!("六十四卦：\t{}\t", bian_gua.to_liu_shi_si_gua().unicode_symbol());
    println!("{}", bian_gua.to_liu_shi_si_gua());

    let hu_gua = ben_gua.hu_gua();
    println!("---");
    println!("互卦（过程）：");
    println!("{}", hu_gua);
    print!("六十四卦：\t{}\t", hu_gua.to_liu_shi_si_gua().unicode_symbol());
    println!("{}", hu_gua.to_liu_shi_si_gua());

    println!("--- 综合占断 ---");

    // 1. 本卦分析
    let rel_ben = ti.wu_xing().relation_with(yong.wu_xing());
    println!("【本卦 - 现状】：{}", rel_ben.description());

    // 2. 互卦分析 (注意：互卦通常看整体生克，这里简化为互上与互下的关系)
    let hu = ben_gua.hu_gua();
    // 互卦通常以“体”对比互卦的上下五行
    println!("【互卦 - 过程】：体卦受{}影响，受{}影响",
             ti.wu_xing().relation_with(hu.shang.wu_xing()).description(),
             ti.wu_xing().relation_with(hu.xia.wu_xing()).description()
    );

    // 3. 变卦分析 (体不变，用变为变卦的动爻所在单卦)
    let bian = ben_gua.bian_gua(dong_yao);
    // 变卦中，“用”卦是发生了变动的那部分
    let (_, new_yong) = bian.ti_yong(dong_yao).unwrap();
    let rel_bian = ti.wu_xing().relation_with(new_yong.wu_xing());
    println!("【变卦 - 结果】：{}", rel_bian.description());

    println!("--- 扩展视角 ---");

    let cuo = ben_gua.cuo_gua();
    println!("错卦（反面）：{}\t{}",
             cuo.to_liu_shi_si_gua().unicode_symbol(),
             cuo.to_liu_shi_si_gua()
    );

    let zong = ben_gua.zong_gua();
    println!("综卦（换位）：{}\t{}",
             zong.to_liu_shi_si_gua().unicode_symbol(),
             zong.to_liu_shi_si_gua()
    );
}
