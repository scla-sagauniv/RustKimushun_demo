#[derive(Debug)]
pub struct CharCode {
    // 2進数の文字列が入っている
    pub char: String,
}

impl CharCode {
    pub fn from_char(c: char) -> CharCode {
        let mut char_level: String = "".to_string();
        match c {
            'A'..='Z' => char_level = format!("{:0>6b}", c as u8 - 'A' as u8),
            'a'..='z' => char_level = format!("{:0>6b}", c as u8 - 'a' as u8 + 26),
            '0'..='9' => char_level = format!("{:0>6b}", c as u8 - '0' as u8 + 52),
            '+' => char_level = format!("{:0>6b}", 62),
            '/' => char_level = format!("{:0>6b}", 63),
            _ => println!(""),
        };
        CharCode { char: char_level }
    }
    pub fn to_char(&self) -> char {
        let u: &[u8; 1] = &[u8::from_str_radix(&self.char, 2).unwrap()];
        let s: &str = std::str::from_utf8(u).unwrap();
        let c: Vec<char> = s.chars().collect();
        return c[0];
    }
}
