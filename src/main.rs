use image::imageops::blur;
use image::imageops::overlay;
use image::imageops::unsharpen;
use image::ImageOutputFormat;
use std::fs::File;
use std::path::Path;

use watercolor_images;

fn main() {
    let mut img = image::open("data/ferris.png")
        .expect("File not found")
        .into_rgba8();

    // TODO: set alpha to 1 everywhere

    let distances = watercolor_images::compute_distance_to_border(&img);

    let max_distance = 2;
    let mut mask_image = watercolor_images::create_mask(&img, max_distance, &distances);

    mask_image = blur(&mask_image, (max_distance as f32) / 2.0);
    watercolor_images::add_noise(&mut mask_image);

    watercolor_images::apply_threshold_on_grey(&mut mask_image, 128);

    watercolor_images::apply_mask(&mut img, &mask_image);

    img = unsharpen(&img, (max_distance as f32) / 2.0, 1);

    // Put alpha to 0.7
    let alpha_value = (0.7 * 255.0) as u8;
    for pixel in img.pixels_mut() {
        pixel.0[3] = alpha_value;
    }

    watercolor_images::add_random_hue_variation(&mut img);

    let path = Path::new("data/output.png");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    img.write_to(&mut file, ImageOutputFormat::Png);
}
