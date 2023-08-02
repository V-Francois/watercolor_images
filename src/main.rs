use image::ImageOutputFormat;
use std::fs::File;
use std::path::Path;

use watercolor_images;

fn main() {
    let mut img = image::open("data/ferris.png")
        .expect("File not found")
        .into_rgba8();

    let distances = watercolor_images::compute_distance_to_border(&img);

    // Replace pixels close to border by white pixels
    let max_distance = 3;
    watercolor_images::set_pixel_close_to_border_to_white(&mut img, max_distance, &distances);

    let path = Path::new("data/output.png");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    img.write_to(&mut file, ImageOutputFormat::Png);
}
