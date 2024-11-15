extern crate color_thief;
extern crate image;

use std::path;

use color_thief::{Color, ColorFormat};

fn get_image_buffer(img: image::DynamicImage) -> (Vec<u8>, ColorFormat) {
    match img {
        image::DynamicImage::ImageRgb8(buffer) => (buffer.to_vec(), color_thief::ColorFormat::Rgb),
        image::DynamicImage::ImageRgba8(buffer) => {
            (buffer.to_vec(), color_thief::ColorFormat::Rgba)
        }
        _ => unreachable!(),
    }
}

#[test]
fn image1() {
    let img = image::open(&path::Path::new("images/photo1.jpg")).unwrap();
    let (buffer, color_type) = get_image_buffer(img);
    let colors = color_thief::get_palette(&buffer, color_type, 10, 10).unwrap();

    assert_eq!(colors[0], Color::new(54, 37, 28)); //  55,  37,  29
    assert_eq!(colors[1], Color::new(216, 196, 137)); // 213, 193, 136
    assert_eq!(colors[2], Color::new(109, 204, 223)); // 110, 204, 223
    assert_eq!(colors[3], Color::new(127, 119, 58)); // 131, 122,  58
    assert_eq!(colors[4], Color::new(43, 125, 149)); //  43, 124, 148
    assert_eq!(colors[5], Color::new(134, 123, 107)); // 156, 175, 121
    assert_eq!(colors[6], Color::new(160, 178, 120)); // 131, 121, 110
    assert_eq!(colors[7], Color::new(167, 199, 221)); // 167, 198, 220
    assert_eq!(colors[8], Color::new(212, 80, 7)); // 213,  75,   8
}

#[test]
fn image2() {
    let img = image::open(&path::Path::new("images/iguana.png")).unwrap();
    let (buffer, color_type) = get_image_buffer(img);
    let colors = color_thief::get_palette(&buffer, color_type, 10, 10).unwrap();

    assert_eq!(colors[0], Color::new(71, 60, 53));
    assert_eq!(colors[1], Color::new(207, 207, 204));
    assert_eq!(colors[2], Color::new(165, 170, 174));
    assert_eq!(colors[3], Color::new(147, 137, 129));
    assert_eq!(colors[4], Color::new(146, 152, 168));
    assert_eq!(colors[5], Color::new(117, 122, 128));
    assert_eq!(colors[6], Color::new(100, 101, 113));
    assert_eq!(colors[7], Color::new(22, 20, 27));
    assert_eq!(colors[8], Color::new(180, 148, 116));
}
