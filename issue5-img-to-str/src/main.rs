use image::DynamicImage;

const BIT_LEN: usize = 21;

#[derive(Debug)]
pub struct ColorCode {
    // 2進数の文字列が入っている
    pub r: String,
    pub g: String,
    pub b: String,
}

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

#[derive(Debug)]
pub struct DisplayCode {
    pub color_code: ColorCode,
    pub char_code: CharCode,
}

fn main() {
    let img: DynamicImage = image::open("cat.png").unwrap();
    // バイト文字列としてエンコード
    let binaries: Vec<u8> = img.into_bytes();
    // バイナリを21文字ずつに分割
    let splitted_binary: Vec<String> = split_binary(binaries);
    // 21文字を構造体に分解
    let made_structures: Vec<DisplayCode> = make_structure(splitted_binary);
}

fn split_binary(binaries: Vec<u8>) -> Vec<String> {
    // バイト文字を結合
    let mut sum_binary: String = "".to_string();
    for binary in binaries {
        let str_binary: String = format!("{:0>8b}", binary);
        sum_binary += &str_binary;
    }

    // 結合されたバイト文字を21文字ずつに分割
    let mut splitted_binary: Vec<String> = Vec::new();
    for chunk in sum_binary.chars().collect::<Vec<char>>().chunks(21) {
        let s: String = chunk.iter().collect();
        splitted_binary.push(s);
    }

    // 最後が21文字かのチェック
    let last_item: &String = splitted_binary.last().unwrap();
    let last_item_len: usize = last_item.chars().count();

    // 長さが21なら何もしない
    if last_item_len == BIT_LEN {
        print!("{}", &last_item_len);
    } else {
        let adjust_zero: String = "0".repeat(BIT_LEN - &last_item_len);
        let new_last_item: String = last_item.to_string() + &adjust_zero;
        splitted_binary.pop();
        splitted_binary.push(new_last_item);
    }
    return splitted_binary;
}

fn make_structure(splitted_binary: Vec<String>) -> Vec<DisplayCode> {
    let mut display_code: Vec<DisplayCode> = Vec::new();
    for binary in splitted_binary {
        // print!("all:{}\n ", binary.to_string());
        // print!("fir{}\n ", binary[0..5].to_string());
        // print!("mid{}\n ", binary[5..10].to_string());
        // print!("fin{}\n\n ", binary[15..].to_string());
        display_code.push(DisplayCode {
            color_code: (ColorCode {
                r: binary[0..5].to_string(),
                g: binary[5..10].to_string(),
                b: binary[10..15].to_string(),
            }),
            char_code: (CharCode {
                char: binary[15..].to_string(),
            }),
        })
    }
    return display_code;
}
