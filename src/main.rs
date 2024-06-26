use axum::{response::{Html, IntoResponse}, extract::{Path, Query}};
use axum::routing::get;
use axum::{Router, Server};
use axum::routing::get_service;
use serde::Deserialize;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

mod error;

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// First Route

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_hello())
        .fallback_service(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 6969));
    println!("->> Listening on http://{}", &addr);
    
    Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
}

// --- Hello handler

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

async fn handler_hello(params: Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");

    Html(format!("Hello <strong>{name}</strong>"))
}
