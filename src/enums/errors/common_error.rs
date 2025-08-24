use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use diesel::r2d2;
use thiserror::Error;
use crate::constants::error_code_const::FAILED_CODE;
use crate::errors::business_error::BusinessError;
use crate::models::responses::response::ApiResponse;

/// 统一的错误类型
#[derive(Error, Debug, Clone)]
pub enum CommonError {
    #[error("System error: {0}")]
    Sys(String),

    #[error("Business error: {0}")]
    Biz(BusinessError),
}

/// 转换 Axum 的错误
impl From<axum::http::Error> for CommonError {
    fn from(err: axum::http::Error) -> Self {
        CommonError::Sys(format!("Axum框架错误: {}", err))
    }
}

/// 转换数据库sql错误
impl From<diesel::result::Error> for CommonError {
    fn from(err: diesel::result::Error) -> Self {
        CommonError::Sys(format!("数据库错误: {}", err))
    }
}

/// 转换数据库连接池错误
impl From<r2d2::Error> for CommonError {
    fn from(err: r2d2::Error) -> Self {
        CommonError::Sys(format!("数据库连接池错误: {}", err))
    }
}

/// 实现 IntoResponse 以返回标准化的 JSON 响应
impl IntoResponse for CommonError {
    fn into_response(self) -> Response {
        let (_, error_response) = match &self {
            CommonError::Biz(business_err) => (
                StatusCode::OK, // 业务异常通常返回 200，但可以根据需要调整
                ApiResponse::<()>::failed(business_err.code, business_err.message.clone()),
            ),
            CommonError::Sys(err_msg) => (
                StatusCode::OK,
                ApiResponse::<()>::failed(FAILED_CODE, err_msg.clone()),
            ),
        };

        // 返回 JSON 响应
        Json(ApiResponse::<()>::failed(error_response.code,
                                       error_response.message.unwrap_or(String::from(""))))
            .into_response()
    }
}