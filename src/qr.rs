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

fn _create_qrcode(content: &str) -> Result<QRCode, String> {
    QRBuilder::new(String::from(content))
        .ecl(ECL::H)
        .build()
        .map_err(|e| {
            // Handle the "Data too big to be encoded" error
            let err_msg = format!("{:?}", e);
            if err_msg.contains("too big") || err_msg.contains("too long") || err_msg.contains("capacity") {
                format!("Data too large to encode in QR code. Maximum capacity with high error correction (ECL::H) is approximately 2953 bytes for version 40. Your data is {} bytes.", content.len())
            } else {
                format!("Failed to generate QR code: {}", err_msg)
            }
        })
}

pub fn qrcode_png(content: &str, shape: &str, size: Option<u32>, embed: &bool) -> Result<Vec<u8>, String> {
    let qrcode = _create_qrcode(content)?;
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
    Ok(builder.shape(_get_shape_from_str(shape))
        .to_pixmap(&qrcode)
        .encode_png()
        .map_err(|e| format!("Failed to encode PNG: {:?}", e))?)
}

pub fn qrcode_svg(content: &str, shape: &str, embed: &bool) -> Result<String, String> {
    let qrcode = _create_qrcode(content)?;
    let mut builder = SvgBuilder::default();
    if embed.to_owned() {
        builder.image(_get_embedded_img());
    };
    Ok(builder.shape(_get_shape_from_str(shape))
        .to_str(&qrcode))
}

#[test]
fn test_qrcode_png() {
    const PNG_MAGIC_BYTES: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    let buffer = qrcode_png("random-string-1234567890", "unknown", None, &false).unwrap();
    assert!(buffer.len() > 8);
    let buf= &buffer[0..8];
    assert!(PNG_MAGIC_BYTES == buf);
}

#[test]
fn test_qrcode_png_embed() {
    const PNG_MAGIC_BYTES: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    let buffer = qrcode_png("random-string-1234567890", "unknown", None, &true).unwrap();
    assert!(buffer.len() > 8);
    let buf= &buffer[0..8];
    assert!(PNG_MAGIC_BYTES == buf);
}

#[test]
fn test_qrcode_png_circle_shape() {
    const PNG_MAGIC_BYTES: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    let buffer = qrcode_png("random-string-1234567890", "CIRCLE", None, &false).unwrap();
    assert!(buffer.len() > 8);
    let buf= &buffer[0..8];
    assert!(PNG_MAGIC_BYTES == buf);
}

#[test]
fn test_qrcode_svg() {
    let svg = qrcode_svg("random-string-0987654321", "diamond", &false).unwrap();
    assert!(!svg.is_empty());
    assert_eq!(&svg[..5], "<svg ");
}

#[test]
fn test_qrcode_too_large() {
    // Create a string that's too large for QR code
    let large_content = "x".repeat(10000);
    let result = qrcode_png(&large_content, "square", None, &false);
    assert!(result.is_err());
    let err_msg = result.unwrap_err();
    assert!(err_msg.contains("too large") || err_msg.contains("capacity"));
}
