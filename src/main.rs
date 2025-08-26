use crate::enums::errors::common_error::CommonError;
use crate::errors::business_error::BusinessError;
use crate::errors::error_types;
use crate::models::responses::response::ApiResponse;
use crate::routes::public::public_auth_api;
use crate::routes::public::public_info_api;
use crate::routes::user::user_api;
use axum::body::Body;
use axum::error_handling::HandleErrorLayer;
use axum::extract::{Path, Query, Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use axum::{Router, middleware};
use clap::Parser;
use diesel::query_dsl::select_dsl::SelectDsl;
use diesel::r2d2::{self, ConnectionManager};
use diesel::{MysqlConnection, RunQueryDsl};
use dotenvy::dotenv;
use serde::Deserialize;
use std::backtrace::Backtrace;
use std::convert::Infallible;
use std::env;
use tokio::net::TcpListener;
use tower::{ServiceBuilder, service_fn};
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::services::ServeDir;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, HttpMakeClassifier, TraceLayer};
use tracing::{Level, Span, error, info, info_span};
use tracing::instrument::WithSubscriber;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};
use uuid::Uuid;

mod constants;
mod core;
mod enums;
mod errors;
mod models;
mod routes;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[tokio::main]
async fn main() {
    // 加载环境变量
    let cli = Cli::parse();
    // 根据 profile 加载对应的 .env 文件
    let env_file = format!(".env.{}", cli.profile);
    dotenvy::from_filename(&env_file).expect("Failed to load .env file");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    // 创建数据库连接池
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .min_idle(Some(5))
        .max_size(15) // 设置最大连接数
        .build(manager)
        .expect("Failed to create database pool");

    // 检查数据库连接
    if let Err(e) = check_for_backend(&pool) {
        error!("Database backend check failed: {}", e);
        return;
    }

    // 创建应用程序状态
    let app_state = AppState { db_pool: pool };

    // 初始化tracing
    init_tracing();

    let routes_all = Router::new()
        .merge(routes_hello(app_state))
        .nest("/user", user_api::apis())
        .nest("/public/base", public_info_api::apis())
        .nest("/public/auth", public_auth_api::apis())
        .layer(ServiceBuilder::new().layer(log_trace_id_layer()))// 日志添加trace_id
        .layer(CatchPanicLayer::new())// 处理panic日志
        .fallback_service(routes_static());

    let addr = "127.0.0.1:3000";
    info!("server start on {addr}");
    axum::serve(
        TcpListener::bind(addr).await.unwrap(),
        routes_all.into_make_service(),
    )
    .await
    .unwrap();
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "dev")]
    profile: String, // 环境
}

#[derive(Clone)]
struct AppState {
    db_pool: DbPool,
}

/// 检查数据库连接的函数
fn check_for_backend(pool: &DbPool) -> Result<(), diesel::result::Error> {
    let mut conn = pool.get().map_err(|e| {
        error!("Failed to get connection from pool: {}", e);
        diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new(e.to_string()),
        )
    })?;

    // 执行简单的查询以检查连接
    diesel::sql_query("SELECT 1")
        .execute(&mut conn)
        .map_err(|e| {
            error!("Failed to execute test query: {}", e);
            e
        })?;

    info!("MySQL backend is reachable");
    Ok(())
}

/// 初始化tracing
fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter("info,tower_http=info")
        .with_file(true)// 打印日志所在文件
        .with_line_number(true)// 打印日志所在行
        .with_ansi(true)// 日志颜色
        .init();
}

/// 初始化trace_id
fn log_trace_id_layer() -> TraceLayer<HttpMakeClassifier, impl Fn(&Request<Body>) -> Span + Clone + Send + Sync + 'static> {
    TraceLayer::new_for_http()
        .make_span_with(|request: &Request<_>| {
            // 尝试从 header 中获取 trace_id，如果没有则生成新的
            let trace_id = request
                .headers()
                .get("x-trace-id")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string())
                .unwrap_or_else(|| Uuid::new_v4().to_string());
            info_span!("",
                trace_id = %trace_id,
                method = %request.method(),
                uri = %request.uri(),
            )
        })
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO))
}

/// 静态文件目录
fn routes_static() -> Router {
    Router::new().nest_service(
        "/assets",
        get_service(ServeDir::new("assets").fallback(service_fn(|_| async {
            Ok::<_, Infallible>(
                Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::from("File not found"))
                    .unwrap(),
            )
        }))),
    )
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

async fn hello_handler(Query(params): Query<HelloParams>) -> Result<String, CommonError> {
    info!("params: {params:?}");
    if params.name.is_none() {
        // return Err(CommonError::Biz(BusinessError::custom("name is empty")));
        panic!("name is empty")
    }
    Ok(format!("Hello World! {}", params.name.unwrap()))
}

async fn hello_path_handler(Path(name): Path<String>) -> impl IntoResponse {
    info!("name: {name:?}");
    Html(format!("Hello World! {name}"))
}

async fn get_users(State(state): State<AppState>) -> Result<String, CommonError> {
    let mut conn = state
        .db_pool
        .get()
        .map_err(|e| {
            CommonError::Sys{
                message: e.to_string(),
                backtrace: Backtrace::capture()
            }
        })?;

    // 假设有一个 users 表，加载所有用户
    use crate::schema::rust_user::dsl::*;
    let results = rust_user
        .select(name)
        .load::<String>(&mut conn)
        .map_err(|e| {
            CommonError::Sys{
                message: e.to_string(),
                backtrace: Backtrace::capture()
            }
        })?;

    Ok(format!("Users: {:?}", results))
}
