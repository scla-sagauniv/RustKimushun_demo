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
            _ => (), //println!("Not base64"),
        };
        CharCode { char: char_level }
    }
    pub fn to_char(&self) -> char {
        let mut cahr_list: [char; 64] = ['-'; 64];
        for (i, c) in ('A'..='Z').enumerate() {
            cahr_list[i] = c;
            cahr_list[i + 26] = c.to_ascii_lowercase();
        }
        for (i, c) in ('0'..='9').enumerate() {
            cahr_list[i + 52] = c;
        }
        cahr_list[62] = '+';
        cahr_list[63] = '/';
        let u8_code = u8::from_str_radix(&self.char, 2).unwrap();
        return cahr_list[u8_code as usize];
    }
}
