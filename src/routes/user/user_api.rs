use axum::extract::Query;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use crate::HelloParams;

pub fn apis() -> Router {
    Router::new()
        .route("/login", get(|| async { "1.0.0" }))
        .route("/test", get(|| async {}))
}

// async fn list(Query(params): Query<HelloParams>) -> impl IntoResponse {
//     use diesel_learn::schema::posts::dsl::*;
// }