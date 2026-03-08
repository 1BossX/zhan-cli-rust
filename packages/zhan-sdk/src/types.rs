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
    pub id: Option<String>, // API 有时没有 id
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
    pub data: T,
}

/// 帖子类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PostType {
    Debug,
    #[serde(rename = "code-review")]
    CodeReview,
    Config,
    Question,
}

/// 创建帖子输入
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePostInput {
    pub title: String,
    #[serde(rename = "contentMd")]
    pub content_md: String,
    #[serde(rename = "type")]
    pub post_type: PostType,
    pub tags: Option<Vec<String>>,
    #[serde(rename = "bountyCents")]
    pub bounty_cents: Option<i64>,
}

/// 创建帖子响应
#[derive(Debug, Clone, serde::Deserialize)]
pub struct CreatePostResponse {
    pub id: String,
    pub url: String,
    #[serde(rename = "estimatedCvs")]
    pub estimated_cvs: f64,
    #[serde(rename = "templateWarnings")]
    pub template_warnings: Option<Vec<String>>,
    #[serde(rename = "privacyWarnings")]
    pub privacy_warnings: Option<Vec<String>>,
}

/// 用户统计
#[derive(Debug, Clone, serde::Deserialize)]
pub struct UserStats {
    #[serde(rename = "postCount")]
    pub post_count: i64,
    #[serde(rename = "avgCvs")]
    pub avg_cvs: f64,
    #[serde(rename = "totalCoffeeCents")]
    pub total_coffee_cents: i64,
    #[serde(rename = "totalSolvedCount")]
    pub total_solved_count: i64,
    #[serde(rename = "recentPostCount")]
    pub recent_post_count: i64,
    pub reputation: f64,
    #[serde(rename = "balanceCents")]
    pub balance_cents: i64,
}

/// Solved 请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolvedInput {
    #[serde(rename = "bountyCents")]
    pub bounty_cents: Option<i64>,
    #[serde(rename = "timeSavedMinutes")]
    pub time_saved_minutes: Option<i64>,
}

/// Solved 响应
#[derive(Debug, Clone, serde::Deserialize)]
pub struct SolvedResponse {
    #[serde(rename = "solvedId")]
    pub solved_id: String,
    #[serde(rename = "postId")]
    pub post_id: String,
    #[serde(rename = "newSolvedCount")]
    pub new_solved_count: i64,
}

/// Reward 请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardInput {
    #[serde(rename = "answererUserId")]
    pub answerer_user_id: String,
}

/// Coffee 输入
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoffeeInput {
    #[serde(rename = "postId")]
    pub post_id: String,
    #[serde(rename = "amountCents")]
    pub amount_cents: Option<i64>,
    pub currency: Option<String>,
}

/// Coffee 响应
#[derive(Debug, Clone, serde::Deserialize)]
pub struct CoffeeResponse {
    #[serde(rename = "coffeeId")]
    pub coffee_id: String,
    #[serde(rename = "checkoutUrl")]
    pub checkout_url: String,
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
