use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use qrcode::QrCode;
use image::Luma;
use image::codecs::png::PngEncoder;
use image::ColorType;
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
    let code = QrCode::new(content).unwrap();
    println!("{}", format!("Qrcode created with version {:?}, ecl {:?}, and width {}", 
        code.version(), code.error_correction_level(), code.width()));
    let image: image::ImageBuffer<Luma<u8>, Vec<u8>> = code.render::<Luma<u8>>().build();
    // https://stackoverflow.com/questions/50731636/how-do-i-encode-a-rust-piston-image-and-get-the-result-in-memory
    // See also https://github.com/enaut/pslink/blob/master/app/src/pages/list_links.rs#L1044 
    let mut buf: Vec<u8> = Vec::new();
    let wd: u32 = image.width();
    let ht: u32 = image.height();
    let encoder: PngEncoder<&mut Vec<u8>> = PngEncoder::new(&mut buf);
    encoder.encode(&image.into_raw(), wd, ht, ColorType::L8).expect("Cannot encode image");
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
