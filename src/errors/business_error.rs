use std::fmt;
use serde::Serialize;
use crate::constants::error_code_const::FAILED_CODE;

/// 业务异常的详细信息
#[derive(Debug, Serialize, Clone)]
pub struct BusinessError {
    pub code: i32,
    pub message: String,
}

impl BusinessError {
    pub fn new(code: i32, message: impl Into<String>) -> Self {
        BusinessError {
            code,
            message: message.into(),
        }
    }

    pub fn custom(message: impl Into<String>) -> Self {
        BusinessError::new(FAILED_CODE, message)
    }

}

/// 为 BusinessError 实现 Display trait
impl fmt::Display for BusinessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "code: {}, message: {}", self.code, self.message)
    }
}