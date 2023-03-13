fn main() {
    let mut lt = leptess::LepTess::new(None, "eng").unwrap();
    lt.set_image("test.png").unwrap();
    println!("{}", lt.get_utf8_text().unwrap());
    // println!("hello!");
    // for n in 1..11 {
    //     println!("{n}Hello, world!");
    // }
}
