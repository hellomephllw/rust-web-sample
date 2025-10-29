use serde::{Deserialize, Serialize};

/// 帖子列表查询参数
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct PostListParam {
    /// 页码，从1开始
    #[serde(default = "default_page_no")]
    pub page_no: u32,
    /// 每页条数
    #[serde(default = "default_page_size")]
    pub page_size: u32,
}

fn default_page_no() -> u32 {
    1
}

fn default_page_size() -> u32 {
    20
}
