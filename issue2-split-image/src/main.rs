use image::{imageops::crop, open, GenericImageView};
use std::path::Path;

fn main() {
    let mut img = open("test.png").unwrap();
    let split_y = 7;
    let h = img.height() / split_y;
    let w = img.width();
    let x = 0;
    let mut y = 0;

    for i in 0..split_y {
        let _image = crop(&mut img, x, y, w, h).to_image();
        let mut img_name = "output/test".to_string();
        img_name += &i.to_string();
        img_name += ".png";

        let path = Path::new(&img_name);
        _image.save(path);
        y += h;
    }

    // println!("hello!");
    // for n in 1..11 {
    //     println!("{n}Hello, world!");
    // }
}
