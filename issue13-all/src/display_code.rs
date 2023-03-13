mod char_code;
mod color_code;
pub use char_code::CharCode;
pub use color_code::{Color, ColorCode};

#[derive(Debug)]
pub struct DisplayCode {
    pub color_code: ColorCode,
    pub char_code: CharCode,
}
