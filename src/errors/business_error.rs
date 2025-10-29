use crate::constants::error_code_const::FAILED_CODE;
use std::backtrace::Backtrace;
use std::fmt;

/// 业务异常的详细信息
#[derive(Debug)]
pub struct BusinessError {
    pub code: i32,
    pub message: String,
    pub backtrace: Backtrace,
}

impl BusinessError {
    pub fn new(code: i32, message: impl Into<String>) -> Self {
        BusinessError {
            code,
            message: message.into(),
            backtrace: Backtrace::capture(),
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
