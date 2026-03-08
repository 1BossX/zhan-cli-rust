use serde::{Deserialize, Serialize};

/// 设备码登录请求响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCodeResponse {
    #[serde(rename = "deviceCode")]
    pub device_code: String,
    #[serde(rename = "userCode")]
    pub user_code: String,
    #[serde(rename = "verificationUri")]
    pub verification_uri: String,
    #[serde(rename = "verificationUriComplete")]
    pub verification_uri_complete: Option<String>,
    pub expires_in: u64,
    #[serde(rename = "interval")]
    pub poll_interval: u64,
}

/// 设备码轮询请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevicePollRequest {
    #[serde(rename = "deviceCode")]
    pub device_code: String,
}

/// 设备码轮询响应 - 成功
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceTokenResponse {
    pub access_token: String,
    #[serde(rename = "tokenType")]
    pub token_type: String,
    pub expires_in: u64,
}

/// 设备码轮询错误响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevicePollError {
    pub error: String,
    #[serde(rename = "errorDescription")]
    pub error_description: Option<String>,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Option<String>,  // API 有时没有 id
    pub username: String,
    pub email: Option<String>,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: Option<String>,
    pub reputation: Option<f64>,
}

/// Feed 帖子
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    #[serde(rename = "authorId")]
    pub author_id: Option<String>,
    #[serde(rename = "author")]
    pub author: Option<User>,
    #[serde(rename = "type")]
    pub post_type: String,
    pub title: String,
    #[serde(rename = "contentMd")]
    pub content_md: Option<String>,
    pub content: Option<String>,
    #[serde(rename = "effectiveChars")]
    pub effective_chars: Option<i64>,
    pub tags: Vec<String>,
    #[serde(rename = "bountyCents")]
    pub bounty_cents: i64,
    #[serde(rename = "bountyStatus")]
    pub bounty_status: Option<String>,
    #[serde(rename = "cvsScore")]
    pub cvs_score: Option<f64>,
    #[serde(rename = "coffeeCount")]
    pub coffee_count: Option<i64>,
    #[serde(rename = "coffeeTotalCents")]
    pub coffee_total_cents: Option<i64>,
    #[serde(rename = "solvedCount")]
    pub solved_count: Option<i64>,
    #[serde(rename = "viewCount")]
    pub view_count: i64,
    #[serde(rename = "likeCount")]
    pub like_count: Option<i64>,
    #[serde(rename = "commentCount")]
    pub comment_count: Option<i64>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
}

/// Feed 响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedResponse {
    pub posts: Vec<Post>,
    #[serde(rename = "nextCursor")]
    pub next_cursor: Option<String>,
    pub total: i64,
}

/// API 响应包装（部分 API 使用）
#[derive(Debug, Clone, serde::Deserialize)]
pub struct DataWrapper<T> {
    pub data: T
}

/// API 通用响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub error: Option<String>,
}

/// 搜索结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub posts: Vec<Post>,
    #[serde(rename = "nextCursor")]
    pub next_cursor: Option<String>,
    pub total: i64,
}

/// 健康检查响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: Option<String>,
}
