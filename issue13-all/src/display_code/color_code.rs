#[derive(Debug)]
pub struct ColorCode {
    // 2進数の文字列が入っている
    pub r: String,
    pub g: String,
    pub b: String,
}

#[derive(Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ColorCode {
    pub fn from_color(color: &Color) -> ColorCode {
        // 32段階なんで　256 / 32 = 8 で１段階当たり 8
        let r_level = color.r / 8;
        let g_level = color.g / 8;
        let b_level = color.b / 8;
        ColorCode {
            r: format!("{:0>5b}", r_level),
            g: format!("{:0>5b}", g_level),
            b: format!("{:0>5b}", b_level),
        }
    }

    pub fn to_color(&self) -> Color {
        // 32段階なんで　256 / 32 = 8 で１段階当たり 8
        Color {
            r: u8::from_str_radix(&self.r, 2).unwrap() * 8,
            g: u8::from_str_radix(&self.g, 2).unwrap() * 8,
            b: u8::from_str_radix(&self.b, 2).unwrap() * 8,
        }
    }
}
