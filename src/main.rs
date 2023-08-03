use image::imageops::blur;
use image::imageops::overlay;
use image::ImageOutputFormat;
use std::fs::File;
use std::path::Path;

use watercolor_images;

fn main() {
    let img = image::open("data/ferris.png")
        .expect("File not found")
        .into_rgba8();

    // TODO: set alpha to 1 everywhere

    let distances = watercolor_images::compute_distance_to_border(&img);

    let max_distance = 3;
    let mut mask_image = watercolor_images::create_mask(&img, max_distance, &distances);

    let (w, h) = mask_image.dimensions();
    let mut noisy_background = watercolor_images::create_noisy_background(w, h, 150);
    noisy_background = blur(&noisy_background, (max_distance as f32) / 2.0);

    overlay(&mut mask_image, &noisy_background, 0, 0);
    mask_image = blur(&mask_image, (max_distance as f32) / 2.0);
    mask_image = blur(&mask_image, (max_distance as f32) / 2.0);

    let path = Path::new("data/output.png");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    mask_image.write_to(&mut file, ImageOutputFormat::Png);
}
