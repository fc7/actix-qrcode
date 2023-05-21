use crate::qr;
use crate::BarcodeParams;
use warp::http::Response;

pub(crate) fn qrcode_body(p: BarcodeParams) -> Response<Vec<u8>> {
    let _render: &str = &p.render.to_owned().unwrap_or(String::from("png"));
    let _shape: &str = &p.shape.to_owned().unwrap_or(String::from("square"));
    if _render == "svg" {
        let svg = qr::qrcode_svg(&p.content, _shape);
        Response::builder()
        .header("Content-Type", "image/svg+xml")
        .header("Content-Length", svg.len())
        .body(svg.into_bytes()).unwrap()
    } else {
        let png: Vec<u8> = qr::qrcode_png(&p.content, _shape, p.size);
        Response::builder()
        .header("Content-Type", "image/png")
        .header("Content-Length", png.len())
        .body(png).unwrap()
    }
}
