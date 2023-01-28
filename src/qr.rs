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
