use image::GenericImageView;
use image::GrayImage;
use image::ImageBuffer;
use image::Luma;
use image::Pixel;
use image::Rgba;
use image::RgbaImage;
use ndarray::Array2;
use noise::{NoiseFn, Perlin, Seedable};
use perlin_noise::PerlinNoise;
use rand;
use rand_distr::{Distribution, Normal};
use std::f32::consts::PI;
use std::ops::Deref;

fn expand_distances(distances: &mut Array2<i32>, iteration: i32) -> bool {
    let mut incremented_distances = false;

    // not guaranteed to have the smallest distances written in the array
    // but that will be enough for now

    let shape = distances.shape();
    let w = shape[0];
    let h = shape[1];

    for x in 0..w {
        for y in 0..h {
            let here = distances[[x as usize, y as usize]];
            if here != iteration {
                continue;
            }

            if x < w - 1 {
                let right = distances[[(x + 1) as usize, y as usize]];
                if right == -1 {
                    distances[[(x + 1) as usize, y as usize]] = iteration + 1;
                    incremented_distances = true;
                }
            }
            if y < h - 1 {
                let below = distances[[x as usize, (y + 1) as usize]];
                if below == -1 {
                    distances[[x as usize, (y + 1) as usize]] = iteration + 1;
                    incremented_distances = true;
                }
            }
            if x > 0 {
                let left = distances[[(x - 1) as usize, y as usize]];
                if left == -1 {
                    distances[[(x - 1) as usize, y as usize]] = iteration + 1;
                    incremented_distances = true;
                }
            }
            if y > 0 {
                let above = distances[[x as usize, (y - 1) as usize]];
                if above == -1 {
                    distances[[x as usize, (y - 1) as usize]] = iteration + 1;
                    incremented_distances = true;
                }
            }
        }
    }

    return incremented_distances;
}

pub fn compute_distance_to_border<P: Pixel, Container: Deref<Target = [P::Subpixel]>>(
    img: &ImageBuffer<P, Container>,
) -> Array2<i32> {
    let (w, h) = img.dimensions();

    // Will contain the distance between any pixel and a border between two colors
    let mut distances = Array2::<i32>::zeros((w as usize, h as usize));
    distances.fill(-1);

    let mut found_border = false;
    for x in 0..(w - 1) {
        for y in 0..h {
            let pixel_here = img.get_pixel(x, y).to_rgba();
            let pixel_right = img.get_pixel(x + 1, y).to_rgba();
            if pixel_here != pixel_right {
                distances[[x as usize, y as usize]] = 0;
                distances[[(x + 1) as usize, y as usize]] = 0;
                found_border = true;
            }
            if y < h - 1 {
                let pixel_bottom = img.get_pixel(x, y + 1).to_rgba();
                if pixel_here != pixel_bottom {
                    distances[[x as usize, y as usize]] = 0;
                    distances[[x as usize, (y + 1) as usize]] = 0;
                    found_border = true;
                }
            }
        }
    }
    if !found_border {
        return distances;
    }

    let mut iteration = 0;
    loop {
        let incremented_distances = expand_distances(&mut distances, iteration);
        iteration += 1;
        if !incremented_distances {
            break;
        }
    }

    return distances;
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
            let diff = perlin.get([x as f64 / 10.0, y as f64 / 10.0]) * 100.0;
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
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        for i in 3..4 {
            if pixel.0[i] > 0 && pixel.0[i] < 255 {
                let diff = perlin.get([
                    (x as usize + i * 5) as f64 / 75.0,
                    (y as usize + i * 5) as f64 / 75.0,
                ]) * 30.0;
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
