use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::env;

pub mod qr;

#[derive(Deserialize)]
pub(crate) struct BarcodeParams {
    content: String,
    size: Option<u32>,
    render: Option<String>, // "png" or "svg", default = png
    shape: Option<String>,  // Square, Circle, RoundedSquare, Vertical, Horizontal, Diamond (case-insensitive)
    embed: Option<bool> // whether to embed an image
}

#[get("/")]
async fn render_qrcode(params: web::Query<BarcodeParams>) -> impl Responder {
    let _render: &str = &params.render.to_owned().unwrap_or(String::from("png"));
    let _shape: &str = &params.shape.to_owned().unwrap_or(String::from("square"));
    let _embed: &bool = &params.embed.to_owned().unwrap_or(false);
    if _render == "svg" {
        let svg = qr::qrcode_svg(&params.content, _shape, _embed);
        HttpResponse::Ok().insert_header(("Content-Type", "image/svg+xml")).body(svg)
    } else {
        let png = qr::qrcode_png(&params.content, _shape, params.size, _embed);
        HttpResponse::Ok().insert_header(("Content-Type", "image/png")).body(png)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const DEFAULT_IP: &'static str = "0.0.0.0";
    const DEFAULT_PORT: &'static str = "8080";
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
            .service(render_qrcode)
            .route(
                "/health/{_:(readiness|liveness)}",
                web::get().to(HttpResponse::Ok),
            )
    })
    .bind(bind_address + ":" + &port)?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use actix_web::{test, App, HttpResponse, web};

    use crate::render_qrcode;

    #[actix_web::test]
    async fn test_render_qrcode_get() {
        let app = 
            test::init_service(App::new().service(render_qrcode)).await;
        let req = test::TestRequest::get().uri("/?content=random-string-123").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        assert_eq!(resp.headers().get("Content-Type").unwrap(), "image/png");
    }

    #[actix_web::test]
    async fn test_render_qrcode_get_with_size() {
        let app = 
            test::init_service(App::new().service(render_qrcode)).await;
        let req = test::TestRequest::get().uri("/?content=random-string-123&size=1000").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        assert_eq!(resp.headers().get("Content-Type").unwrap(), "image/png");
    }

    #[actix_web::test]
    async fn test_render_qrcode_get_with_shape() {
        let app = 
            test::init_service(App::new().service(render_qrcode)).await;
        let req = test::TestRequest::get().uri("/?content=lucy-in-the-sky-with-diamonds&shape=dIamOnD").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        assert_eq!(resp.headers().get("Content-Type").unwrap(), "image/png");
    }

    #[actix_web::test]
    async fn test_render_qrcode_get_svg() {
        let app = 
            test::init_service(App::new().service(render_qrcode)).await;
        let req = test::TestRequest::get().uri("/?content=random-string-123&render=svg").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        assert_eq!(resp.headers().get("Content-Type").unwrap(), "image/svg+xml");
    }

    #[actix_web::test]
    async fn test_probes() {
        let app = 
            test::init_service(App::new().route(
                "/health/{_:(readiness|liveness)}",
                web::get().to(HttpResponse::Ok),
            )).await;
        let req = test::TestRequest::get().uri("/health/liveness").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_render_qrcode_get_empty() {
        let app = 
            test::init_service(App::new().service(render_qrcode)).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }
}