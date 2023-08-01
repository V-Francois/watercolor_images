use image::GenericImageView;

use watercolor_images;

#[test]
fn test_read_image() {
    let img = image::open("data/ferris.png").expect("File not found");
    let (w, h) = img.dimensions();
    assert_eq!(w, 880);
    assert_eq!(h, 587);
}

#[test]
fn test_compute_distance_to_border() {
    let img = image::open("data/ferris.png")
        .expect("File not found")
        .into_rgba16();

    let distances = watercolor_images::compute_distance_to_border(&img);
    let shape = distances.shape();
    assert_eq!(shape, &[880, 587]);

    let zero_value: Option<&i32> = Some(0).as_ref();
    assert_eq!(distances.iter().max(), zero_value);
}
