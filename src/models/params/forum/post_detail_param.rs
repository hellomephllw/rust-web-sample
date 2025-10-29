use serde::{Deserialize, Serialize};

/// 帖子详情查询参数
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct PostDetailParam {
    /// 帖子ID
    pub post_id: i64,
}

