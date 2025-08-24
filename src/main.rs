use std::backtrace::Backtrace;
use std::convert::Infallible;
use std::env;
use axum::response::{Html, IntoResponse, Response};
use axum::{middleware, Router};
use axum::body::Body;
use axum::error_handling::HandleErrorLayer;
use axum::extract::{Path, Query, Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::routing::{get, get_service};
use clap::Parser;
use diesel::{MysqlConnection, RunQueryDsl};
use diesel::query_dsl::select_dsl::SelectDsl;
use diesel::r2d2::{self, ConnectionManager};
use dotenvy::dotenv;
use serde::Deserialize;
use tokio::net::TcpListener;
use tower::{service_fn, ServiceBuilder};
use tower_http::services::ServeDir;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, HttpMakeClassifier, TraceLayer};
use tracing::{error, info, info_span, Level, Span};
use uuid::Uuid;
use crate::routes::public::public_auth_api;
use crate::routes::public::public_info_api;
use crate::routes::user::user_api;
use crate::enums::errors::common_error::CommonError;
use crate::errors::business_error::BusinessError;
use crate::errors::error_types;
use crate::models::responses::response::ApiResponse;

mod routes;
mod models;
mod schema;
mod core;
mod enums;
mod constants;
mod errors;

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[tokio::main]
async fn main() {
    let biz_err = BusinessError::custom(String::from("abc"));
    println!("{:?}", biz_err);
    tracing::error!("报错: {:?}", biz_err);
    // 加载环境变量
    let cli = Cli::parse();
    // 根据 profile 加载对应的 .env 文件
    let env_file = format!(".env.{}", cli.profile);
    dotenvy::from_filename(&env_file).expect("Failed to load .env file");

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    // 创建数据库连接池
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .min_idle(Some(5))
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
        .layer(ServiceBuilder::new().layer(log_trace_layer()))
        // .layer(middleware::from_fn(log_trace_layer()))
        // .layer(middleware::from_fn(error_middleware))
        .fallback_service(routes_static());

    let addr = "127.0.0.1:3000";
    info!("server start on {addr}");
    axum::serve(
        TcpListener::bind(addr).await.unwrap(),
        routes_all.into_make_service()
    ).await.unwrap();
}


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "dev")]
    profile: String,// 环境
}

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

/// 错误处理中间件
async fn error_middleware(
    req: axum::http::Request<Body>,
    next: Next,
) -> Result<Response, CommonError> {
    // 先运行下一个服务，捕获可能的错误
    let response = next.run(req).await;

    // if let Some(common_error) = response.extensions().get::<ApiResponse<_>>() {
    //     tracing::info!("Info: {:?}", common_error);
    // }

    // 检查响应是否包含 AppError（通过扩展）
    if let Some(common_error) = response.extensions().get::<CommonError>() {
        match common_error {
            CommonError::Biz(business_err) => {
                error!("Business error: {}", business_err);
                // 业务异常不打印堆栈
            }
            CommonError::Sys(msg) => {
                error!("System error: {}", msg);
                // 打印简化的堆栈
                let backtrace = Backtrace::capture();
                error!("Backtrace:\n{}", backtrace);
                // let backtrace_lines: Vec<&str> = backtrace
                //     .to_string()
                //     .lines()
                //     .filter(|line| !line.contains("axum::") && !line.contains("tokio::"))
                //     .collect();
                // error!("Simplified backtrace:\n{}", backtrace_lines.join("\n"));
            }
        }
        // 返回原始响应（已通过 IntoResponse 转换为 HTTP 200）
        return Ok(response);
    }

    // 如果响应状态不是 200，记录为系统错误
    if !response.status().is_success() {
        error!("Unexpected error: status={}", response.status());
        let backtrace = std::backtrace::Backtrace::capture();
        error!("Backtrace:\n{}", backtrace);
        // let backtrace_lines: Vec<&str> = backtrace
        //     .to_string()
        //     .lines()
        //     .filter(|line| !line.contains("axum::") && !line.contains("tokio::"))
        //     .collect();
        // error!("Simplified backtrace:\n{}", backtrace_lines.join("\n"));
        // 返回新的 HTTP 200 响应
        let api_response = ApiResponse::<()>::failed(10000, format!("Unexpected error: status {}", response.status()));
        return Ok(api_response.into_response());
    }

    Ok(response)

    // // 如果 response 表示错误（例如非 2xx 状态码），可以进一步处理
    // if response.status().is_server_error() || response.status().is_client_error() {
    //     // 假设错误信息在扩展中（可选，视情况实现）
    //     error!("Error response detected: status={}", response.status());
    //     // 这里可以进一步提取错误信息并转换为 AppError
    //     return Err(CommonError::Sys(format!("Unexpected error: status {}", response.status())));
    // }
    //
    // Ok(response)
    // // 如果已经是 AppError，直接返回
    // if let Some(app_error) = err.downcast_ref::<CommonError>() {
    //     // 打印简化的错误信息
    //     match app_error {
    //         CommonError::Biz(business_err) => {
    //             error!("Business error: code={}, message={}", business_err.code, business_err.message);
    //         }
    //         CommonError::Sys(msg) => {
    //             error!("System error: {}", msg);
    //             // 可选：打印简化的堆栈
    //             let backtrace = Backtrace::capture();
    //             error!("Simplified backtrace:\n{}", backtrace);
    //             // let backtrace_lines: Vec<&str> = backtrace
    //             //     .to_string()
    //             //     .lines()
    //             //     .filter(|line| !line.contains("axum::") && !line.contains("tokio::")) // 过滤无关堆栈
    //             //     .collect();
    //             // error!("Simplified backtrace:\n{}", backtrace_lines.join("\n"));
    //         }
    //     }
    //     return Err(app_error.clone());
    // }
    //
    // // 其他未处理的错误转为系统异常
    // let system_error = CommonError::Sys(format!("Unexpected error: {}", err));
    // error!("Unexpected error: {}", err);
    // Err(system_error)
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

async fn hello_handler(Query(params): Query<HelloParams>) -> Result<String, CommonError> {
    params.name.as_deref().unwrap_or("abc");
    info!("params: {params:?}");
    if params.name.is_none() {
        return Err(CommonError::Biz(BusinessError::custom("name is empty")));
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
        .map_err(|e| CommonError::Sys(String::from("数据库连接错误")))?;

    // 假设有一个 users 表，加载所有用户
    use crate::schema::rust_user::dsl::*;
    let results = rust_user
        .select(name)
        .load::<String>(&mut conn)
        .map_err(|e| CommonError::Sys(String::from("数据库查询错误")))?;

    Ok(format!("Users: {:?}", results))
}