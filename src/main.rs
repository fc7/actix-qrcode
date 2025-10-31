use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
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
async fn render_qrcode(req: HttpRequest, params: web::Query<BarcodeParams>) -> impl Responder {
    // Log the incoming request
    log::info!(
        "QR Code request - IP: {}, User-Agent: {}, Content: {}, Render: {}, Shape: {}, Size: {:?}",
        req.connection_info().realip_remote_addr().unwrap_or("unknown"),
        req.headers().get("user-agent").map(|h| h.to_str().unwrap_or("unknown")).unwrap_or("unknown"),
        params.content,
        params.render.as_ref().unwrap_or(&"png".to_string()),
        params.shape.as_ref().unwrap_or(&"square".to_string()),
        params.size
    );
    
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

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
    
    log::info!("Starting QRCode microservice");
    
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
    
    log::info!("Binding to {}:{}", bind_address, port);
    
    // Configure CORS based on environment variable
    let cors_origin = env::var("CORS_ORIGIN").ok();
    if let Some(ref origin) = cors_origin {
        log::info!("CORS configured for origin: {}", origin);
    } else {
        log::info!("CORS configured to allow any origin");
    }
    
    HttpServer::new(move || {
        let cors = if let Some(ref origin) = cors_origin {
            Cors::default()
                .allowed_origin(origin)
                .allow_any_method()
                .allow_any_header()
                .max_age(3600)
        } else {
            Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
                .max_age(3600)
        };
        
        App::new()
            .wrap(cors)
            .service(render_qrcode)
            .route(
                "/health/{probe:(readiness|liveness)}",
                web::get().to(health_check),
            )
    })
    .bind(bind_address + ":" + &port)?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use actix_web::{test, App, web};

    use crate::{render_qrcode, health_check};

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
                "/health/{probe:(readiness|liveness)}",
                web::get().to(health_check),
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