use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::env;

pub mod qr;

#[derive(Deserialize)]
struct BarcodeParams {
    content: String,
    size: Option<u32>,
    //TODO type: "png" or "svg" (default=png)
}

#[get("/qrcode")]
async fn render_qrcode(params: web::Query<BarcodeParams>) -> impl Responder {
    //TODO let _size = params.size.unwrap_or(600);
    let png = qr::qrcode_png(&params.content, params.size);
    HttpResponse::Ok().insert_header(("Content-Type", "image/png")).body(png)
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
