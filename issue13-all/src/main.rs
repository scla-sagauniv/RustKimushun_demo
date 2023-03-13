use std::io::Cursor;

use image::DynamicImage;

mod display_code;
use display_code::CharCode;
use display_code::ColorCode;
use display_code::DisplayCode;

const BIT_LEN: usize = 21;
fn main() {
    let origin_img = image::open("cat.png").unwrap();
    to_code(
        origin_img.width(),
        origin_img.height(),
        origin_img.as_bytes(),
    );
    let img = image::open("suku.png").unwrap();
    reconstruction(img.width(), img.height(), &img.as_bytes());
}

fn to_code(width: u32, height: u32, img_bytes: &[u8]) -> Vec<DisplayCode> {
    // ここをImageRgba8にするかImageRgb8にするかで結果がかわる
    let img = DynamicImage::ImageRgba8(
        image::ImageBuffer::from_vec(width, height, img_bytes.to_vec()).unwrap(),
    );
    // バイト文字列としてエンコード
    let binaries: Vec<u8> = img.into_bytes();
    // バイナリを21文字ずつに分割
    let splitted_binary: Vec<String> = split_binary(binaries);
    // 21文字を構造体に分解
    make_structure(splitted_binary)
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

fn reconstruction(width: u32, height: u32, img_bytes: &[u8]) {
    // ここをImageRgba8にするかImageRgb8にするかで結果がかわる
    let mut img = DynamicImage::ImageRgba8(
        image::ImageBuffer::from_vec(width, height, img_bytes.to_vec()).unwrap(),
    );
    let images = img_split(&mut img);
    for (i, image) in images.iter().enumerate() {
        let text = ocr(image);
        println!("OCR{i}::\n{text}\n");
    }
}

fn ocr(img: &DynamicImage) -> String {
    let mut lt = leptess::LepTess::new(None, "eng").unwrap();
    let mut buf: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Png)
        .unwrap();
    lt.set_image_from_mem(&buf).unwrap();
    format!("{}", lt.get_utf8_text().unwrap())
}

fn img_split(img: &mut DynamicImage) -> Vec<DynamicImage> {
    let split_y = 7;
    println!("{}", img.height());
    println!("{}", img.width());
    let h = img.height() / split_y;
    let w = img.width();
    let x = 0;
    let mut y = 0;

    let mut img_list = vec![];
    for _ in 0..split_y {
        let _image = image::imageops::crop(img, x, y, w, h).to_image();
        y += h;
        let (width, height) = _image.dimensions();
        let pixels = _image.into_raw();
        let image_buffer = image::ImageBuffer::from_raw(width, height, pixels).unwrap();
        img_list.push(DynamicImage::ImageRgba8(image_buffer));
    }
    img_list
}
