use std::backtrace::Backtrace;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use diesel::r2d2;
use tracing::{error};
use crate::constants::error_code_const::FAILED_CODE;
use crate::errors::business_error::BusinessError;
use crate::models::responses::response::ApiResponse;

/// 统一的错误类型
#[derive(Debug)]
pub enum CommonError {
    Sys {
        message: String,
        backtrace: Backtrace,
    },
    Biz(BusinessError),
}

/// 转换 Axum 的错误
impl From<axum::http::Error> for CommonError {
    fn from(err: axum::http::Error) -> Self {
        CommonError::Sys {
            message: format!("Axum框架错误: {}", err),
            backtrace: Backtrace::capture()
        }
    }
}

/// 转换数据库sql错误
impl From<diesel::result::Error> for CommonError {
    fn from(err: diesel::result::Error) -> Self {
        CommonError::Sys {
            message: format!("数据库错误: {}", err),
            backtrace: Backtrace::capture(),
        }
    }
}

/// 转换数据库连接池错误
impl From<r2d2::Error> for CommonError {
    fn from(err: r2d2::Error) -> Self {
        CommonError::Sys {
            message: format!("数据库连接池错误: {}", err),
            backtrace: Backtrace::capture(),
        }
    }
}

/// 实现 IntoResponse 以返回标准化的 JSON 响应
impl IntoResponse for CommonError {
    fn into_response(self) -> Response {
        let (_status_code, error_response, backtrace) = match self {
            CommonError::Sys {message, backtrace} => (
                StatusCode::OK,
                ApiResponse::<()>::failed(FAILED_CODE, message),
                backtrace,
            ),
            CommonError::Biz(business_err) => (
                StatusCode::OK, // 业务异常通常返回 200，但可以根据需要调整
                ApiResponse::<()>::failed(business_err.code, business_err.message),
                business_err.backtrace,
            ),
        };
        error!(error = ?error_response, backtrace = %format!("{:#?}", backtrace), "|-Error occurred-| ");
        // 返回 JSON 响应
        Json(ApiResponse::<()>::failed(error_response.code,
                                       error_response.message.unwrap_or(String::from(""))))
            .into_response()
    }
}