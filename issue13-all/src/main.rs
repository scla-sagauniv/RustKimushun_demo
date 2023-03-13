use std::io::Cursor;

use image::DynamicImage;

fn main() {
    let img = image::open("suku.png").unwrap();
    reconstruction(img.width(), img.height(), &img.as_bytes());
}

fn reconstruction(width: u32, height: u32, img_bytes: &[u8]) {
    // ここをImageRgba8にするかImageRgb8にするかで結果がかわる
    let mut img = DynamicImage::ImageRgba8(
        image::ImageBuffer::from_vec(width, height, img_bytes.to_vec()).unwrap(),
    );
    let images = img_split(&mut img);
    for (i, image) in images.iter().enumerate() {
        image.save(format!("{i}.png"));
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
