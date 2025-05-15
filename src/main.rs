use std::convert::Infallible;
use axum::response::{Html, IntoResponse, Response};
use axum::{Router};
use axum::body::Body;
use axum::extract::{Path, Query, Request};
use axum::http::StatusCode;
use axum::routing::{get, get_service};
use serde::Deserialize;
use tower::service_fn;
use tower_http::services::ServeDir;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{info, info_span, Level};
use uuid::Uuid;

mod web;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("info,tower_http=debug")
        .init();

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(|request: &Request<_>| {
            let trace_id = Uuid::new_v4(); // 生成唯一 traceId
            info_span!("http_request",
                trace_id = %trace_id,
                method = %request.method(),
                uri = %request.uri(),
            )
    })
        .on_request(DefaultOnRequest::new().level(Level::INFO))  // 确保级别匹配
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    let routes_all = Router::new()
        .merge(routes_hello())
        .layer(trace_layer)
        .fallback_service(routes_static());

    let addr = "127.0.0.1:3000";
    info!("server start on {addr}");
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        routes_all.into_make_service()
    ).await.unwrap();
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
    info!("params: {params:?}");
    Html("Hello World!")
}

async fn hello_path_handler(Path(name): Path<String>) -> impl IntoResponse {
    info!("name: {name:?}");
    Html(format!("Hello World! {name}"))
}