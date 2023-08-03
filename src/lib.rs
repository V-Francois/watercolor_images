use image::GenericImageView;
use image::GrayAlphaImage;
use image::ImageBuffer;
use image::LumaA;
use image::Pixel;
use image::Rgba;
use image::RgbaImage;
use ndarray::Array2;
use rand;
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

pub fn create_mask(img: &RgbaImage, max_distance: i32, distances: &Array2<i32>) -> GrayAlphaImage {
    let (w, h) = img.dimensions();
    let mut mask_image = GrayAlphaImage::new(w, h);

    let max_value: u8 = 255;
    let white_pixel = LumaA([max_value, max_value]);
    let black_pixel = LumaA([0 as u8, max_value]);

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

pub fn set_pixel_close_to_border_to_white(
    img: &mut RgbaImage,
    max_distance: i32,
    distances: &Array2<i32>,
) {
    let (w, h) = img.dimensions();
    let max_value: u8 = 255;
    let white_pixel = Rgba([max_value, max_value, max_value, max_value]);
    for x in 0..w {
        for y in 0..h {
            let local_dist = distances[[x as usize, y as usize]];
            if local_dist < max_distance {
                let gap = max_distance - local_dist;
                if (gap == 1 && rand::random::<f32>() > 0.5) | (gap > 1) {
                    img.put_pixel(x, y, white_pixel);
                }
            }
        }
    }
}

pub fn create_noisy_background(width: u32, height: u32, max_value: u8) -> GrayAlphaImage {
    let width_of_tilable = (rand::random::<f32>() * 10.0 + 10.0) as u32;
    let height_of_tilable = (rand::random::<f32>() * 10.0 + 10.0) as u32;
    let mut tile = GrayAlphaImage::new(width_of_tilable, height_of_tilable);

    // black points with an alpha channel that is low
    for x in 0..width_of_tilable {
        let sin_val_x = ((x as f32 / width_of_tilable as f32) * 2.0 * PI).sin();
        for y in 0..height_of_tilable {
            let sin_val_y = ((y as f32 / height_of_tilable as f32) * 2.0 * PI).sin();
            let alpha_value = (sin_val_x * sin_val_y * max_value as f32
                + rand::random::<f32>() * max_value as f32 / 3.0)
                .abs();
            tile.put_pixel(x, y, LumaA([0 as u8, alpha_value as u8]));
        }
    }

    let mut background = GrayAlphaImage::new(width, height);
    for x in 0..width {
        for y in 0..height {
            let pixel = tile.get_pixel(x % width_of_tilable, y % height_of_tilable);
            background.put_pixel(x, y, *pixel);
        }
    }

    return background;
}
