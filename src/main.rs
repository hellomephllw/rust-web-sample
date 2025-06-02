use std::convert::Infallible;
use std::env;
use axum::response::{Html, IntoResponse, Response};
use axum::{Router};
use axum::body::Body;
use axum::extract::{Path, Query, Request, State};
use axum::http::StatusCode;
use axum::routing::{get, get_service};
use diesel::{MysqlConnection, RunQueryDsl};
use diesel::query_dsl::select_dsl::SelectDsl;
use diesel::r2d2::{self, ConnectionManager};
use dotenvy::dotenv;
use serde::Deserialize;
use tower::{service_fn, ServiceBuilder};
use tower_http::services::ServeDir;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, HttpMakeClassifier, TraceLayer};
use tracing::{info, info_span, Level, Span};
use uuid::Uuid;
use crate::api::public::public_auth_api;
use crate::api::public::public_info_api;
use crate::api::user::user_api;

mod api;
mod model;
mod schema;
mod core;

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[derive(Clone)]
struct AppState {
    db_pool: DbPool,
}

// 检查数据库连接的函数
fn check_for_backend(pool: &DbPool) -> Result<(), diesel::result::Error> {
    let mut conn = pool.get().map_err(|e| {
        eprintln!("Failed to get connection from pool: {}", e);
        diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new(e.to_string()),
        )
    })?;

    // 执行简单的查询以检查连接
    diesel::sql_query("SELECT 1")
        .execute(&mut conn)
        .map_err(|e| {
            eprintln!("Failed to execute test query: {}", e);
            e
        })?;

    println!("MySQL backend is reachable");
    Ok(())
}

#[tokio::main]
async fn main() {
    // 加载环境变量
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    // 创建数据库连接池
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .max_size(15) // 设置最大连接数
        .build(manager)
        .expect("Failed to create database pool");

    // 检查数据库连接
    if let Err(e) = check_for_backend(&pool) {
        eprintln!("Database backend check failed: {}", e);
        return;
    }

    // 创建应用程序状态
    let app_state = AppState { db_pool: pool };

    tracing_subscriber::fmt()
        .with_env_filter("info,tower_http=info")
        .init();

    let routes_all = Router::new()
        .merge(routes_hello(app_state))
        .nest("/user", user_api::apis())
        .nest("/public/base", public_info_api::apis())
        .nest("/public/auth", public_auth_api::apis())
        .layer(ServiceBuilder::new()
            .layer(log_trace_layer()))
        .fallback_service(routes_static());

    let addr = "127.0.0.1:3000";
    info!("server start on {addr}");
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        routes_all.into_make_service()
    ).await.unwrap();
}
fn log_trace_layer() -> TraceLayer<HttpMakeClassifier, impl Fn(&Request<Body>) -> Span + Clone + Send + Sync + 'static> {
    TraceLayer::new_for_http()
        .make_span_with(|request: &Request<_>| {
            let trace_id = Uuid::new_v4();
            info_span!("http_request",
                trace_id = %trace_id,
                method = %request.method(),
                uri = %request.uri(),
            )
        })
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO))
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

fn routes_hello(app_state: AppState) -> Router {
    Router::new()
        .route("/hello", get(hello_handler))
        .route("/hello/:name", get(hello_path_handler))
        .route("/hello/users", get(get_users))
        .with_state(app_state)
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

async fn get_users(State(state): State<AppState>) -> Result<String, StatusCode> {
    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 假设有一个 users 表，加载所有用户
    use crate::schema::rust_user::dsl::*;
    let results = rust_user
        .select(name)
        .load::<String>(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(format!("Users: {:?}", results))
}