#![deny(warnings)]
use warp::Filter;
use serde::{Deserialize};
use std::{env, net::SocketAddr, str::FromStr};
pub mod qr;
pub mod handlers;

#[derive(Deserialize)]
pub(crate) struct BarcodeParams {
    content: String,
    size: Option<u32>,
    render: Option<String>, // "png" or "svg", default = png
    shape: Option<String>,  // Square, Circle, RoundedSquare, Vertical, Horizontal, Diamond (case-insensitive)
}

#[tokio::main]
async fn main() {

    let qrcode = warp::get()
        .and(warp::path::end()) // filter only applies to root
        .and(warp::query::<BarcodeParams>())
        .map(handlers::qrcode_body);
    let qrcode_post = warp::post()
        .and(warp::path::end())
        .and(warp::body::json())
        .map(handlers::qrcode_body);
    let health = warp::get()
        .and(warp::path("health"))
        .and(
            warp::path("readiness").or(warp::path("liveness"))
        ).map(|_| "OK");

    let routes = qrcode.or(qrcode_post).or(health);
    
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
    let addr : String = bind_address + ":" + &port;
    warp::serve(routes).run(SocketAddr::from_str(&addr).unwrap()).await;
}