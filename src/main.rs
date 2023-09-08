use std::env;
use image::io::Reader as ImageReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut img = ImageReader::open(file_path)
        .expect("Not an Image").decode()
        .expect("Decode Failed");
    image::imageops::invert(&mut img);
    img.save(file_path).unwrap();
}
