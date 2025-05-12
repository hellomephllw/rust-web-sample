use std::convert::Infallible;
use std::net::SocketAddr;
use axum::response::{Html, IntoResponse, Response};
use axum::{Router};
use axum::body::Body;
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::routing::{get, get_service};
use serde::Deserialize;
use tower::service_fn;
use tower_http::services::ServeDir;

mod web;

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_hello())
        .fallback_service(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("server start on {addr}");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
}

fn routes_static() -> Router {
    Router::new().nest_service("/assets", get_service(ServeDir::new("assets").fallback(service_fn(|_| async {
        Ok::<_, Infallible>(
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("File not found"))
                .unwrap())
    }))))
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(hello_handler))
        .route("/hello/:name", get(hello_path_handler))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn hello_handler(Query(params): Query<HelloParams>) -> impl IntoResponse {
    params.name.as_deref().unwrap_or("abc");
    println!("params: {params:?}");
    Html("Hello World!")
}

async fn hello_path_handler(Path(name): Path<String>) -> impl IntoResponse {
    println!("name: {name:?}");
    Html(format!("Hello World! {name}"))
}