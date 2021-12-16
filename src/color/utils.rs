/// 输入RGB显示颜色字体
/// 自定义颜色测试
pub fn custom(c1: u8, c2: u8, c3: u8) {
    println!(
        "\x1b[38;2;{};{};{}m{}\x1b[0m",
        c1, c2, c3, "This is custom Color"
    );
}
