use axum::routing::get;
use axum::Router;

pub fn apis() -> Router {
    Router::new().route("/rsaPub", get(|| async { "xxxxx" }))
}
