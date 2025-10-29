use async_trait::async_trait;

use crate::core::RootState;
use crate::enums::errors::common_error::CommonError;
use crate::models::entities::forum::post::Post;

#[async_trait]
pub trait PostService {
    /// 获取帖子列表
    /// @param root_state 应用状态
    /// @param page_no 页码
    /// @param page_size 每页条数
    /// @return 帖子列表
    /// @throws CommonError 通用错误
    async fn list(&self, root_state: &RootState, page_no: u32, page_size: u32) -> Result<Vec<Post>, CommonError>;

    /// 获取帖子详情
    /// @param root_state 应用状态
    /// @param post_id 帖子ID
    /// @return 帖子详情
    /// @throws CommonError 通用错误
    async fn detail(&self, root_state: &RootState, post_id: i64) -> Result<Post, CommonError>;

    /// 创建帖子
    /// @param root_state 应用状态
    /// @param post 帖子
    /// @return 帖子
    /// @throws CommonError 通用错误
    async fn create(&self, root_state: &RootState, post: Post) -> Result<Post, CommonError>;
}
