use std::fmt;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use crate::constants::error_code_const::FAILED_CODE;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {

    pub fn new(code: i32, data: Option<T>, message: Option<String>) -> Self {
        ApiResponse { code, data, message }
    }

    pub fn success(data: Option<T>) -> Self {
        ApiResponse::new(FAILED_CODE, data, None)
    }

    pub fn failed(code: i32, message: String) -> Self {
        ApiResponse::new(code, None, Some(message))
    }

    pub fn failed_with_data(code: i32, data: Option<T>, message: String) -> Self {
        ApiResponse::new(code, data, Some(message))
    }

}

/// 实现 IntoResponse 以返回标准化的 JSON 响应
impl<T> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        // 返回 JSON 响应
        Json(ApiResponse::<()>::failed(self.code,
                                       self.message.unwrap_or(String::from(""))))
            .into_response()
    }

}

impl fmt::Display for ApiResponse<()> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}