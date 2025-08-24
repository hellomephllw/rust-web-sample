use crate::errors::business_error::BusinessError;

pub const INVALID_USERNAME_OR_PASSWORD_CODE: i32 = 10001;
pub const TOKEN_EXPIRED_CODE: i32 = 10002;

pub fn invalid_username_or_password() -> BusinessError {
    BusinessError::new(INVALID_USERNAME_OR_PASSWORD_CODE, "用户名或密码错误")
}
pub fn token_expired() -> BusinessError {
    BusinessError::new(TOKEN_EXPIRED_CODE, "token已过期")
}