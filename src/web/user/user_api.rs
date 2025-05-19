use axum::Router;
use axum::routing::get;

pub fn apis() -> Router {
    Router::new().route("/login", get(|| async { "1.0.0" }))
}