use std::env::args;

mod interactive;

fn main() {
    let path = args().nth(1).expect("no path specified");
    let size = args()
        .nth(2)
        .expect("no size specified")
        .parse::<f32>()
        .expect("invalid size");
    let image = image::open(&path)
        .unwrap()
        .resize(200, 200, image::imageops::FilterType::Nearest)
        .to_rgb8();

    interactive::run(&image, size);
}
