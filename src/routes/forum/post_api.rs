use axum::{extract::State, Json, Router};
use axum::routing::post;
use crate::core::root_state::RootState;
use crate::models::entities::forum::post::Post;
use crate::models::params::forum::{PostListParam, PostDetailParam};
use crate::models::responses::response::ApiResponse;
use crate::services::forum::post_service::PostService;
use crate::services::forum::post_service_impl::PostServiceImpl;

/// 帖子列表处理函数
async fn list(State(root_state): State<RootState>, Json(params): Json<PostListParam>) -> Json<ApiResponse<Vec<Post>>> {
    let service = PostServiceImpl::new();
    
    match service.list(&root_state, params.page_no, params.page_size).await {
        Ok(posts) => Json(ApiResponse::success(Some(posts))),
        Err(err) => {
            Json(ApiResponse::failed("业务错误".to_string()))
        },
    }
}

/// 帖子详情处理函数
async fn detail(State(root_state): State<RootState>, Json(params): Json<PostDetailParam>) -> Json<ApiResponse<Post>> {
    let service = PostServiceImpl::new();
    
    match service.detail(&root_state, params.post_id).await {
        Ok(post) => Json(ApiResponse::success(Some(post))),
        Err(err) => {
            Json(ApiResponse::failed("业务错误".to_string()))
        }
    }
}

/// 帖子路由
pub fn apis() -> Router<RootState> {
    Router::new()
        .route("/list", post(list))
        .route("/detail", post(detail))
}
