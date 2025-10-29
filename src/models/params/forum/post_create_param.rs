use serde::Deserialize;

/// 创建帖子参数
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PostCreateParam {
    /// 帖子标题
    pub title: String,
    /// 帖子内容
    pub content: String,
    /// 是否发布
    #[serde(default = "default_published")]
    pub published: bool,
}

fn default_published() -> bool {
    false
}

