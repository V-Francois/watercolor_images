use image::{GenericImageView};

#[test]
fn test_read_image(){
    let img = image::open("data/ferris.png").expect("File not found");
    let (w, h) = img.dimensions();
    assert_eq!(w, 880);
    assert_eq!(h, 587);
}
