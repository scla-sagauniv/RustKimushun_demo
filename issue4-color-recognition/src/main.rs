fn main() {
    let mut lt = leptess::LepTess::new(None, "eng").unwrap();
    lt.set_image("test.png").unwrap();
    println!("{}", lt.get_utf8_text().unwrap());
    let boxes = lt
        .get_component_boxes(leptess::capi::TessPageIteratorLevel_RIL_SYMBOL, true)
        .unwrap();

    let mut img = image::open("test.png").unwrap();
    for (i, b) in boxes.into_iter().enumerate() {
        println!("{:?}", b);
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
    }
}
