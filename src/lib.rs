use image::ImageBuffer;
use image::Pixel;
use ndarray::Array2;
use std::ops::Deref;

pub fn compute_distance_to_border<P: Pixel, Container: Deref<Target = [P::Subpixel]>>(
    img: &ImageBuffer<P, Container>,
) -> Array2<i32> {
    let (w, h) = img.dimensions();
    let mut distances = Array2::<i32>::zeros((w as usize, h as usize));
    distances.fill(-1);
    return distances;
}
