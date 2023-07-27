use image;

#[test]
fn test_read_image(){
    let img = image::open("data/ferris.png").expect("File not found");
}
