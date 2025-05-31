use axum::Router;
use axum::routing::get;

pub fn apis() -> Router {
    Router::new().route("/rsaPub", get(|| async { "xxxxx" }))
}