use image::GrayImage;
use image::Luma;
use image::Pixel;
use image::RgbaImage;
use image::{Rgb, Rgba};
use ndarray::Array2;
use noise::{NoiseFn, Perlin};

pub fn create_masks(img: &RgbaImage) -> (Vec<Rgba<u8>>, Vec<GrayImage>) {
    let (w, h) = img.dimensions();

    // Will label each pixel based on its color
    let mut pixel_labels = Array2::<usize>::zeros((w as usize, h as usize));

    let mut pixel_types = Vec::new();

    for x in 0..w {
        for y in 0..h {
            let pixel_here = img.get_pixel(x, y).to_rgba();
            if x == 0 && y == 0 {
                pixel_labels[[0, 0]] = 0;
                pixel_types.push(pixel_here);
            } else {
                let mut found_pixel = false;
                // Check if pixel is similar to one in the list
                for (i, pixel) in pixel_types.iter().enumerate() {
                    if *pixel == pixel_here {
                        pixel_labels[[x as usize, y as usize]] = i;
                        found_pixel = true;
                        break;
                    }
                }
                if !found_pixel {
                    pixel_labels[[x as usize, y as usize]] = pixel_types.len();
                    pixel_types.push(pixel_here);
                }
            }
        }
    }

    let mut masks: Vec<GrayImage> = Vec::new();
    let mut final_pixel_types = Vec::new();
    let max_value: u8 = 255;
    let white_pixel = Luma([max_value]);
    let black_pixel = Luma([0 as u8]);

    for i in 0..pixel_types.len() {
        let mut mask_image = GrayImage::new(w, h);
        let mut count = 0;
        for x in 0..w {
            for y in 0..h {
                if pixel_labels[[x as usize, y as usize]] == i as usize {
                    count += 1;
                    mask_image.put_pixel(x, y, black_pixel);
                } else {
                    mask_image.put_pixel(x, y, white_pixel);
                }
            }
        }
        // only consider masks with at least 1K points
        if count >= 1000 {
            masks.push(mask_image);
            final_pixel_types.push(pixel_types[i]);
        }
    }
    return (final_pixel_types, masks);
}

pub fn create_mask(img: &RgbaImage, max_distance: i32, distances: &Array2<i32>) -> GrayImage {
    let (w, h) = img.dimensions();
    let mut mask_image = GrayImage::new(w, h);

    let max_value: u8 = 255;
    let white_pixel = Luma([max_value]);
    let black_pixel = Luma([0 as u8]);

    for x in 0..w {
        for y in 0..h {
            let local_dist = distances[[x as usize, y as usize]];
            if local_dist < max_distance {
                mask_image.put_pixel(x, y, black_pixel);
            } else {
                mask_image.put_pixel(x, y, white_pixel);
            }
        }
    }
    return mask_image;
}

pub fn apply_mask(img: &mut RgbaImage, mask: &GrayImage) {
    let (w, h) = img.dimensions();
    let (w2, h2) = mask.dimensions();
    assert_eq!(w, w2);
    assert_eq!(h, h2);
    let max_value: u8 = 255;
    let white_pixel = Rgba([max_value, max_value, max_value, max_value]);
    for x in 0..w {
        for y in 0..h {
            let mask_pixel = mask.get_pixel(x, y);
            if mask_pixel.0[0] == 0 {
                img.put_pixel(x, y, white_pixel);
            }
        }
    }
}

pub fn apply_threshold_on_grey(img: &mut GrayImage, threshold_value: u8) {
    for pixel in img.pixels_mut() {
        if pixel.0[0] > threshold_value {
            pixel.0[0] = 255;
        } else {
            pixel.0[0] = 0;
        }
    }
}

pub fn add_noise(img: &mut GrayImage) {
    let (w, h) = img.dimensions();
    let perlin = Perlin::new(0);

    for x in 0..w {
        for y in 0..h {
            let diff = perlin.get([x as f64 / 15.0, y as f64 / 15.0]) * 100.0;
            let pixel = img.get_pixel(x, y);
            let mut new_value = pixel.0[0] as f64 + diff;
            if new_value < 0.0 {
                new_value = 0.0;
            } else if new_value > 255.0 {
                new_value = 255.0;
            }
            //println!("Old: {}, new: {}", pixel.0[0], new_value);
            img.put_pixel(x, y, Luma([new_value as u8]));
        }
    }
}

pub fn add_random_hue_variation(img: &mut RgbaImage) {
    let perlin = Perlin::new(0);
    let max_value: u8 = 255;
    let white_pixel = Rgb([max_value, max_value, max_value]);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        if (*pixel).to_rgb() == white_pixel {
            continue;
        }
        for i in 0..3 {
            if pixel.0[i] > 0 && pixel.0[i] < 255 {
                let diff = perlin.get([
                    (x as usize + i * 5) as f64 / 75.0,
                    (y as usize + i * 5) as f64 / 75.0,
                ]) * 20.0;
                let mut new_value = pixel.0[i] as f64 + diff;
                if new_value < 0.0 {
                    new_value = 0.0;
                } else if new_value > 255.0 {
                    new_value = 255.0;
                }
                pixel.0[i] = new_value as u8;
            }
        }
    }
}

pub fn transform_mask_into_image(mask: GrayImage, pixel: Rgba<u8>) -> RgbaImage {
    let (w, h) = mask.dimensions();
    let mut image = RgbaImage::new(w, h);

    let max_value: u8 = 255;
    let alpha_value: u8 = (max_value as f32 * 0.7) as u8;
    let white_pixel = Rgba([max_value, max_value, max_value, 0]);
    let mut pixel_to_put = pixel;
    pixel_to_put.0[3] = alpha_value;
    for x in 0..w {
        for y in 0..h {
            let mask_pixel = mask.get_pixel(x, y);
            if mask_pixel.0[0] == max_value {
                image.put_pixel(x, y, white_pixel);
            } else {
                image.put_pixel(x, y, pixel_to_put);
            }
        }
    }

    return image;
}
