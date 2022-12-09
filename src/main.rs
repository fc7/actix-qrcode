use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
// use fast_qr::convert::ConvertError;
use fast_qr::convert::{image::ImageBuilder, Builder, Shape};
use fast_qr::qr::QRBuilder;
use serde::Deserialize;
use std::env;
use std::vec::Vec;

#[derive(Deserialize)]
struct BarcodeParams {
    content: String,
}

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

#[get("/qrcode")]
async fn render_qrcode(params: web::Query<BarcodeParams>) -> impl Responder {
    let png = qrcode_png(&params.content);
    HttpResponse::Ok().insert_header(("Content-Type", "image/png")).body(png)
}

fn qrcode_png(content: &String) -> Vec<u8> {
    let qrcode = QRBuilder::new(content.into())
        .build()
        .unwrap();
    let buf = ImageBuilder::default()
        .shape(Shape::Square).fit_width(600).to_pixmap(&qrcode).encode_png().unwrap();
    buf
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const DEFAULT_IP: &'static str = "127.0.0.1";
    const DEFAULT_PORT: &'static str = "8089";
    let bind_address: String = if env::var("BIND_ADDRESS").is_err() {
        DEFAULT_IP.to_string()
    } else {
        env::var("BIND_ADDRESS").unwrap()
    };
    let port: String = if env::var("PORT").is_err() {
        DEFAULT_PORT.to_string()
    } else {
        env::var("PORT").unwrap()
    };
    HttpServer::new(|| {
        App::new()
            // .app_data(web::QueryConfig::default())
            .service(render_qrcode)
    })
    .bind(bind_address + ":" + &port)?
    .run()
    .await
}
