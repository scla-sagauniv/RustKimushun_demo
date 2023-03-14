use std::io::Cursor;
use std::vec;

use image::DynamicImage;

mod display_code;
use display_code::CharCode;
use display_code::DisplayCode;
use display_code::{Color, ColorCode};
use maplit::hashmap;

fn main() {
    let origin_img = image::open("cat.png").unwrap();
    // println!("===origin===\n{:?}", origin_img.as_bytes());
    let display_cords = to_code(
        origin_img.width(),
        origin_img.height(),
        origin_img.as_bytes(),
    );
    // for a_display_code in display_cords.iter() {
    //     println!(
    //         "color={:?}::char={}",
    //         a_display_code.color_code.to_color(),
    //         a_display_code.char_code.to_char()
    //     )
    // }
    let img = image::open("suku.png").unwrap();
    reconstruction(img.width(), img.height(), &img.as_bytes())
}

fn to_code(width: u32, height: u32, img_bytes: &[u8]) -> Vec<DisplayCode> {
    // ここをImageRgba8にするかImageRgb8にするかで結果がかわる
    let img = DynamicImage::ImageRgb8(
        image::ImageBuffer::from_vec(width, height, img_bytes.to_vec()).unwrap(),
    );
    // バイト文字列としてエンコード
    let binaries: Vec<u8> = img.into_bytes();
    // バイナリを21文字ずつに分割
    let mut splitted_binary: Vec<String> = split_binary(binaries);
    // widthを0番目
    splitted_binary.insert(0, format!("{:0>21b}", width));
    // heightを1番目
    splitted_binary.insert(1, format!("{:0>21b}", height));
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

    // 最後を21文字に変換
    // let last_item: u8 = splitted_binary.last().unwrap().parse().unwrap();
    println!("{}", splitted_binary.last().unwrap());
    let last_item: u32 = u32::from_str_radix(&splitted_binary.last().unwrap(), 2).unwrap();
    let adjusted_last_item: String = format!("{:0>21b}", last_item);

    splitted_binary.pop();
    splitted_binary.push(adjusted_last_item);
    return splitted_binary;
}

// fn adjust_bit_len(lack_bit: String) -> String {
//     const CORRECT_BIT_LEN: usize = 21;
//     let bit_len: usize = lack_bit.chars().count();
//     let mut correct_bit: String;
//     // 長さが21なら何もしない
//     if bit_len == CORRECT_BIT_LEN {
//         println!("oh, you are 21! -> {:?}", &bit_len);
//         correct_bit = lack_bit;
//     } else {
//         let adjust_zero: String = "0".repeat(CORRECT_BIT_LEN - &bit_len);
//         correct_bit = adjust_zero.to_string() + &lack_bit.to_string();
//     }
//     return correct_bit;
// }

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
    let mut all_img_bytes_str = String::from("");
    for (i, mut a_image) in images.iter().enumerate() {
        all_img_bytes_str += &make_img_bytes_str(&mut a_image);
    }
    let (size_bytes, img_bytes_str) = all_img_bytes_str.split_at(42);
    let (width_bytes, height_bytes) = size_bytes.split_at(21);
    let orig_width = u8::from_str_radix(width_bytes, 10).unwrap();
    let orig_height = u8::from_str_radix(height_bytes, 10).unwrap();
    let mut img_bytes: Vec<u8> = vec![];
    for chunk in img_bytes_str.as_bytes().chunks(8) {
        let chunk_str = std::str::from_utf8(chunk).unwrap();
        let chunk_u8_radix = u8::from_str_radix(&chunk_str, 2).unwrap();
        img_bytes.push(chunk_u8_radix);
    }
    let img_result = DynamicImage::ImageRgb8(
        image::ImageBuffer::from_raw(orig_width, orig_height, img_bytes).unwrap(),
    );
    img_result.save("result.png");
}

fn make_img_bytes_str(img: &DynamicImage) -> String {
    let mut img_bytes_str = String::from("");

    let mut lt = leptess::LepTess::new(None, "eng").unwrap();
    let mut buf: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Png)
        .unwrap();
    lt.set_image_from_mem(&buf).unwrap();

    let text = format!("{}", lt.get_utf8_text().unwrap());
    let boxes = lt
        .get_component_boxes(leptess::capi::TessPageIteratorLevel_RIL_SYMBOL, true)
        .unwrap();

    let background_range = hashmap! {
        "r".to_owned() => hashmap! {"min".to_owned() => 200, "max".to_owned() => 255},
        "g".to_owned() => hashmap! {"min".to_owned() => 200, "max".to_owned() => 255},
        "b".to_owned() => hashmap! {"min".to_owned() => 200, "max".to_owned() => 255},
    };
    let mut c_img = img.clone();
    for (i, b) in boxes.into_iter().enumerate() {
        let abox = b.get_geometry();
        let _image = image::imageops::crop(
            &mut c_img,
            abox.x.try_into().unwrap(),
            abox.y.try_into().unwrap(),
            abox.w.try_into().unwrap(),
            abox.h.try_into().unwrap(),
        )
        .to_image();

        let mut r_pixels = vec![];
        let mut g_pixels = vec![];
        let mut b_pixels = vec![];
        for x in 0.._image.width() {
            for y in 0.._image.height() {
                let p = _image.get_pixel(x, y);
                let r = p[0];
                let g = p[1];
                let b = p[2];
                // そのpixelがバックグラウンドか
                if background_range["r"]["min"] < r
                    && r <= background_range["r"]["max"]
                    && background_range["g"]["min"] < g
                    && g <= background_range["g"]["max"]
                    && background_range["b"]["min"] < b
                    && b <= background_range["b"]["max"]
                {
                    continue;
                }
                r_pixels.push(r);
                g_pixels.push(g);
                b_pixels.push(b);
            }
        }
        let mut r_sum: i64 = 0;
        let mut g_sum: i64 = 0;
        let mut b_sum: i64 = 0;
        for i in 0..r_pixels.len() {
            r_sum += r_pixels[i] as i64;
            g_sum += g_pixels[i] as i64;
            b_sum += b_pixels[i] as i64;
        }
        let color = Color {
            r: u8::try_from(r_sum / r_pixels.len() as i64).unwrap(),
            g: u8::try_from(g_sum / g_pixels.len() as i64).unwrap(),
            b: u8::try_from(b_sum / b_pixels.len() as i64).unwrap(),
        };
        let a_color_code = ColorCode::from_color(&color);
        let a_char_code = CharCode::from_char(text.chars().nth(i).unwrap());
        img_bytes_str += &a_color_code.r;
        img_bytes_str += &a_color_code.g;
        img_bytes_str += &a_color_code.b;
        img_bytes_str += &a_char_code.char;
    }
    img_bytes_str
}

fn img_split(img: &mut DynamicImage) -> Vec<DynamicImage> {
    let split_y = 7;
    // println!("{}", img.height());
    // println!("{}", img.width());
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
