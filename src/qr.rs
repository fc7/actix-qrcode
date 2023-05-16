use fast_qr::QRCode;
use fast_qr::convert::{image::ImageBuilder, svg::SvgBuilder, Builder, Shape};
use fast_qr::qr::QRBuilder;
use std::vec::Vec;

fn _get_shape_from_str(shape: &str) -> Shape {
    match shape.to_ascii_lowercase().as_str() {
        //Square,Circle,RoundedSquare,Vertical,Horizontal,Diamond
        "square" => Shape::Square,
        "circle" => Shape::Circle,
        "roundedsquare" => Shape::RoundedSquare,
        "vertical" => Shape::Vertical,
        "horizontal" => Shape::Horizontal,
        "diamond" => Shape::Diamond,
        _ => Shape::Square
    }
}

fn _create_qrcode(content: &str) -> QRCode {
    QRBuilder::new(String::from(content)).build().unwrap()
}

pub fn qrcode_png(content: &str, shape: &str, size: Option<u32>) -> Vec<u8> {
    let qrcode = _create_qrcode(content);
    let mut builder = ImageBuilder::default();
    if size.is_some() {
        builder.fit_width(size.unwrap());
    };
    builder.shape(_get_shape_from_str(shape))
        .to_pixmap(&qrcode)
        .encode_png().unwrap()
}

pub fn qrcode_svg(content: &str, shape: &str) -> String {
    let qrcode = _create_qrcode(content);
    SvgBuilder::default()
    .shape(_get_shape_from_str(shape))
    .to_str(&qrcode)
}

#[test]
fn test_qrcode_png() {
    const PNG_MAGIC_BYTES: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    let buffer: Vec<u8> = qrcode_png("random-string-1234567890", "unknown", None);
    assert!(buffer.len() > 8);
    let buf= &buffer[0..8];
    assert!(PNG_MAGIC_BYTES == buf);
}

#[test]
fn test_qrcode_png_circle_shape() {
    const PNG_MAGIC_BYTES: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    let buffer: Vec<u8> = qrcode_png("random-string-1234567890", "CIRCLE", None);
    assert!(buffer.len() > 8);
    let buf= &buffer[0..8];
    assert!(PNG_MAGIC_BYTES == buf);
}

#[test]
fn test_qrcode_svg() {
    let svg : String = qrcode_svg("random-string-0987654321", "diamond");
    assert!(!svg.is_empty());
    assert_eq!(&svg[..5], "<svg ");
}
