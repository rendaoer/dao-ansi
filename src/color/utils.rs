pub struct HexColor(u8, u8, u8);

impl HexColor {
    /// New HexColor Object
    pub fn new(c1: u8, c2: u8, c3: u8) -> HexColor {
        return HexColor(c1, c2, c3);
    }
    /// Enter the hexadecimal code to print the string:输入16进制代码打印字符串
    pub fn print(&self) {
        print!(
            "\x1b[38;2;{};{};{}m{}\x1b[0m",
            self.0, self.1, self.2, "This is custom Color"
        );
    }
    /// Enter the hexadecimal code to print the string:输入16进制代码打印字符串
    pub fn println(&self) {
        println!(
            "\x1b[38;2;{};{};{}m{}\x1b[0m",
            self.0, self.1, self.2, "This is custom Color"
        );
    }
    ///Formatted return string:格式化返回字符串
    pub fn format(&self, text: &str) -> String {
        format!("\x1b[38;2;{};{};{}m{}\x1b[0m", self.0, self.1, self.2, text)
    }
}
