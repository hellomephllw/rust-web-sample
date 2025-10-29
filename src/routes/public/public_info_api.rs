use axum::routing::get;
use axum::Router;

pub fn apis() -> Router {
    Router::new().route("/appVersion", get(|| async { "1.0.0" }))
}
