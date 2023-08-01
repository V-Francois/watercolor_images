use image::ImageBuffer;
use image::ImageOutputFormat;
use image::Pixel;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use watercolor_images;

fn main() {
    let mut img = image::open("data/ferris.png")
        .expect("File not found")
        .into_rgba16();

    let (w, h) = img.dimensions();

    let distances = watercolor_images::compute_distance_to_border(&img);

    // Replace pixels close to border by white pixels
    let max_distance = 3;

    let max_value: u16 = 65535;
    let white_pixel = Pixel::from_channels(max_value, max_value, max_value, max_value);
    for x in 0..w {
        for y in 0..h {
            let local_dist = distances[[x as usize, y as usize]];
            if local_dist < max_distance {
                img.put_pixel(x, y, white_pixel);
            }
        }
    }

    let path = Path::new("data/output.png");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    img.write_to(&mut file, ImageOutputFormat::Png);
}
