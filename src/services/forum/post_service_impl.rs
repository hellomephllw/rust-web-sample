use crate::core::RootState;
use crate::enums::errors::common_error::CommonError;
use crate::errors::business_error::BusinessError;
use crate::models::entities::forum::post::Post;
use crate::services::forum::post_service::PostService;
use async_trait::async_trait;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

/// 帖子服务实现
pub struct PostServiceImpl;

impl PostServiceImpl {
    /// 创建新的服务实例
    pub fn new() -> Self {
        PostServiceImpl
    }
}

impl Default for PostServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PostService for PostServiceImpl {
    async fn list(&self, _root_state: &RootState, _page_no: u32, _page_size: u32) -> Result<Vec<Post>, CommonError> {
        // TODO: 实现查询数据库的逻辑
        // 暂时返回空列表
        Ok(vec![])
    }

    async fn detail(&self, root_state: &RootState, post_id: i64) -> Result<Post, CommonError> {
        // TODO: 实现查询数据库的逻辑
        let mut conn = root_state.db_pool.get()?;
        use crate::schema::rust_post::dsl::*;
        let post = rust_post
            .filter(id.eq(post_id))
            .first::<Post>(&mut conn)?;
        Ok(post)
    }

    async fn create(&self, _root_state: &RootState, _post: Post) -> Result<Post, CommonError> {
        // TODO: 实现插入数据库的逻辑
        // 暂时返回错误
        Err(CommonError::Biz(BusinessError::custom("Create post not implemented yet")))
    }
}