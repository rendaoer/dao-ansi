use dao_ansi::color::kinds::{BackgroundColor, ForegroundColor, PrimaryColor};
fn main() {
    let pc = PrimaryColor {
        foreground_color: ForegroundColor::Red,
        background_color: BackgroundColor::Blue,
    };
    pc.print("MaoWeiFan!")
}
