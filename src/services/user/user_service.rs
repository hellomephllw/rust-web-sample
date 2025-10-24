pub trait UserService: Send + Sync {
    /// 获取用户信息
    fn get_user(&self, user_id: i32) -> Option<String>;
}