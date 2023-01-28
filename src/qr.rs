use fast_qr::convert::{image::ImageBuilder, Builder, Shape};
use fast_qr::qr::QRBuilder;
use std::vec::Vec;

pub fn qrcode_png(content: &str, size: Option<u32>) -> Vec<u8> {
    let qrcode = QRBuilder::new(content.into()).build().unwrap();
    let mut builder = ImageBuilder::default();
    builder.shape(Shape::Square);
    if size.is_some() {
        builder.fit_width(size.unwrap());
    }
    let buf = builder.to_pixmap(&qrcode).encode_png().unwrap();
    buf
}

#[test]
fn test_qrcode_png() {
    const PNG_MAGIC_BYTES: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    let buffer: Vec<u8> = qrcode_png("random-string-1234567890", Some(600));
    assert!(buffer.len() > 8);
    let buf= &buffer[0..8];
    assert!(PNG_MAGIC_BYTES == buf);
}
