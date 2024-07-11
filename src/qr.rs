use fast_qr::{QRCode, ECL};
use fast_qr::convert::{image::ImageBuilder, svg::SvgBuilder, Builder, Shape, ImageBackgroundShape};
use fast_qr::qr::QRBuilder;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::vec::Vec;
use base64::prelude::*;


static IMG_CONTENT: &str = include_str!("../assets/thehat.svg");

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

fn _get_embedded_img() -> String {
    if env::var("EMBEDDED_IMG_PATH").is_err() {
        String::from("data:image/svg+xml;base64,") 
        + &BASE64_STANDARD.encode(IMG_CONTENT)
    } else {
        let path = env::var("EMBEDDED_IMG_PATH").unwrap();
        String::from("data:image/svg+xml;base64,") 
        + &BASE64_STANDARD.encode(fs::read_to_string(PathBuf::from(path)).unwrap())
    }
}

fn _create_qrcode(content: &str) -> QRCode {
    QRBuilder::new(String::from(content))
    .ecl(ECL::H)
    .build().unwrap()
}

pub fn qrcode_png(content: &str, shape: &str, size: Option<u32>, embed: &bool) -> Vec<u8> {
    let qrcode = _create_qrcode(content);
    let mut builder = ImageBuilder::default();
    if size.is_some() {
        builder.fit_width(size.unwrap());
    };
    if embed.to_owned() {
        builder
        // .background_color([255, 255, 255, 255])
        .image(_get_embedded_img())
        .image_background_shape(ImageBackgroundShape::Square);
        // .image_size(15f64, 2f64)
        // .image_position(37f64 / 2f64, 0f64)
        // .image_background_color([255, 255, 255, 255])
    };
    builder.shape(_get_shape_from_str(shape))
        .to_pixmap(&qrcode)
        .encode_png().unwrap()
}

pub fn qrcode_svg(content: &str, shape: &str, embed: &bool) -> String {
    let qrcode = _create_qrcode(content);
    let mut builder = SvgBuilder::default();
    if embed.to_owned() {
        builder.image(_get_embedded_img());
    };
    builder.shape(_get_shape_from_str(shape))
    .to_str(&qrcode)
}

#[test]
fn test_qrcode_png() {
    const PNG_MAGIC_BYTES: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    let buffer: Vec<u8> = qrcode_png("random-string-1234567890", "unknown", None, &false);
    assert!(buffer.len() > 8);
    let buf= &buffer[0..8];
    assert!(PNG_MAGIC_BYTES == buf);
}

#[test]
fn test_qrcode_png_embed() {
    const PNG_MAGIC_BYTES: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    let buffer: Vec<u8> = qrcode_png("random-string-1234567890", "unknown", None, &true);
    assert!(buffer.len() > 8);
    let buf= &buffer[0..8];
    assert!(PNG_MAGIC_BYTES == buf);
}

#[test]
fn test_qrcode_png_circle_shape() {
    const PNG_MAGIC_BYTES: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    let buffer: Vec<u8> = qrcode_png("random-string-1234567890", "CIRCLE", None, &false);
    assert!(buffer.len() > 8);
    let buf= &buffer[0..8];
    assert!(PNG_MAGIC_BYTES == buf);
}

#[test]
fn test_qrcode_svg() {
    let svg : String = qrcode_svg("random-string-0987654321", "diamond", &false);
    assert!(!svg.is_empty());
    assert_eq!(&svg[..5], "<svg ");
}
