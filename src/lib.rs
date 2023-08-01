use image::ImageBuffer;
use image::Pixel;
use ndarray::Array2;
use std::ops::{Deref, DerefMut};

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
