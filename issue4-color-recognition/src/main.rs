mod color_code;

use image::ColorType;
use maplit::hashmap;

use crate::color_code::{Color, ColorCode};

fn main() {
    let mut lt = leptess::LepTess::new(None, "eng").unwrap();
    lt.set_image("test.png").unwrap();
    println!("{}", lt.get_utf8_text().unwrap());
    let boxes = lt
        .get_component_boxes(leptess::capi::TessPageIteratorLevel_RIL_SYMBOL, true)
        .unwrap();

    let background_range = hashmap! {
        "r".to_owned() => hashmap! {"min".to_owned() => 200, "max".to_owned() => 255},
        "g".to_owned() => hashmap! {"min".to_owned() => 200, "max".to_owned() => 255},
        "b".to_owned() => hashmap! {"min".to_owned() => 200, "max".to_owned() => 255},
    };
    let mut img = image::open("test.png").unwrap();
    for (i, b) in boxes.into_iter().enumerate() {
        print!("");
        let abox = b.get_geometry();
        let _image = image::imageops::crop(
            &mut img,
            abox.x.try_into().unwrap(),
            abox.y.try_into().unwrap(),
            abox.w.try_into().unwrap(),
            abox.h.try_into().unwrap(),
        )
        .to_image();

        let mut img_name = "test".to_string();
        img_name += &i.to_string();
        img_name += ".png";
        _image.save(img_name);

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
        println!("{i}: {:?} {:?}", a_color_code, a_color_code.to_color());
    }
}
