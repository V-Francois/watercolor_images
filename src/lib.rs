use image::ImageBuffer;
use image::Pixel;
use ndarray::Array2;
use std::ops::Deref;

pub fn compute_distance_to_border<P: Pixel, Container: Deref<Target = [P::Subpixel]>>(
    img: &ImageBuffer<P, Container>,
) -> Array2<i32> {
    let (w, h) = img.dimensions();

    // Will contain the distance between any pixel and a border between two colors
    let mut distances = Array2::<i32>::zeros((w as usize, h as usize));
    distances.fill(-1);

    let mut found_border = false;
    for x in 0..(w - 1) {
        for y in 0..(h - 1) {
            let pixel_here = img.get_pixel(x, y).to_rgba();
            let pixel_right = img.get_pixel(x + 1, y).to_rgba();
            if pixel_here != pixel_right {
                distances[[x as usize, y as usize]] = 0;
                distances[[(x + 1) as usize, y as usize]] = 0;
                found_border = true;
            }
            let pixel_bottom = img.get_pixel(x, y + 1).to_rgba();
            if pixel_here != pixel_bottom {
                distances[[x as usize, y as usize]] = 0;
                distances[[x as usize, (y + 1) as usize]] = 0;
                found_border = true;
            }
        }
    }
    if !found_border {
        return distances;
    }

    return distances;
}
