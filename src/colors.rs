
pub struct RgbColor {
    value: (u8, u8, u8),
}


pub enum Color {
    Black(RgbColor),
    White(RgbColor),
}
