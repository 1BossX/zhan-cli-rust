use anyhow::{Context, Result};
use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;

use crate::config::Config;
use crate::types::*;

/// API 客户端错误
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("请求失败: {0}")]
    RequestFailed(String),
    #[error("认证失败，请先登录")]
    NotAuthenticated,
    #[error("API 返回错误: {0}")]
    ApiError(String),
    #[error("网络错误: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("响应解析失败: {0}")]
    ParseError(String),
    #[error("服务器错误: {0}")]
    ServerError(String),
    #[error("请求超时")]
    Timeout,
    #[error("无效的 URL: {0}")]
    InvalidUrl(String),
}

impl From<serde_json::Error> for ApiError {
    fn from(e: serde_json::Error) -> Self {
        ApiError::ParseError(e.to_string())
    }
}

/// API 客户端
#[derive(Debug)]
pub struct ApiClient {
    client: Client,
    base_url: String,
    token: Option<String>,
}

impl ApiClient {
    /// 创建新的 API 客户端
    pub fn new() -> Result<Self> {
        let config = Config::load().context("加载配置失败")?;
        Ok(Self {
            client: Client::builder().timeout(Duration::from_secs(30)).build()?,
            base_url: config.api_url,
            token: config.token,
        })
    }

    /// 使用指定配置创建客户端
    pub fn with_config(config: &Config) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
            base_url: config.api_url.clone(),
            token: config.token.clone(),
        }
    }

    /// 设置访问令牌
    pub fn with_token(mut self, token: String) -> Self {
        self.token = Some(token);
        self
    }

    /// 获取基础 URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// 检查是否有令牌
    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    /// 发送 GET 请求
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, ApiError> {
        let url = format!("{}{}", self.base_url, path);

        let mut request = self.client.get(&url);

        if let Some(token) = &self.token {
            request = request.bearer_auth(token);
        }

        let response = request.send().await?;

        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(ApiError::NotAuthenticated);
        }

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(ApiError::ApiError(error_text));
        }

        // 先获取文本，再解析
        let text = response.text().await?;

        // 尝试解析
        match serde_json::from_str::<T>(&text) {
            Ok(data) => Ok(data),
            Err(e) => {
                // 返回解析错误
                Err(ApiError::ApiError(format!("JSON 解析错误: {}", e)))
            }
        }
    }

    /// 发送 POST 请求
    pub async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, ApiError> {
        let url = format!("{}{}", self.base_url, path);
        let mut request = self.client.post(&url).json(body);

        if let Some(token) = &self.token {
            request = request.bearer_auth(token);
        }

        let response = request.send().await?;

        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(ApiError::NotAuthenticated);
        }

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(ApiError::ApiError(error_text));
        }

        let data = response.json().await?;
        Ok(data)
    }

    /// 健康检查
    pub async fn health(&self) -> Result<HealthResponse, ApiError> {
        self.get::<HealthResponse>("/health").await
    }

    /// 获取当前用户信息
    pub async fn get_current_user(&self) -> Result<User, ApiError> {
        let response: DataWrapper<User> = self.get("/users/me").await?;
        Ok(response.data)
    }

    /// 获取 Feed
    pub async fn get_feed(
        &self,
        r#type: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Vec<Post>, ApiError> {
        let mut path = String::from("/feed?");
        if let Some(t) = r#type {
            path.push_str(&format!("type={}&", t));
        }
        if let Some(l) = limit {
            path.push_str(&format!("limit={}", l));
        }

        let response: DataWrapper<FeedResponse> = self.get(&path).await?;
        Ok(response.data.posts)
    }

    /// 搜索帖子
    pub async fn search(
        &self,
        query: &str,
        r#type: Option<&str>,
        since: Option<u32>,
        limit: Option<u32>,
    ) -> Result<SearchResult, ApiError> {
        let mut path = format!("/search?q={}", query);
        if let Some(t) = r#type {
            path.push_str(&format!("&type={}", t));
        }
        if let Some(s) = since {
            path.push_str(&format!("&since={}", s));
        }
        if let Some(l) = limit {
            path.push_str(&format!("&limit={}", l));
        }

        let response: DataWrapper<SearchResult> = self.get(&path).await?;
        Ok(response.data)
    }

    /// 查看帖子详情
    pub async fn get_post(&self, post_id: &str) -> Result<Post, ApiError> {
        let path = format!("/posts/{}", post_id);
        let response: DataWrapper<Post> = self.get(&path).await?;
        Ok(response.data)
    }

    /// 创建帖子
    pub async fn create_post(
        &self,
        input: &CreatePostInput,
    ) -> Result<CreatePostResponse, ApiError> {
        let response: CreatePostResponse = self.post("/posts", input).await?;
        Ok(response)
    }

    /// 获取用户统计
    pub async fn get_stats(&self) -> Result<UserStats, ApiError> {
        let response: DataWrapper<UserStats> = self.get("/users/me/stats").await?;
        Ok(response.data)
    }

    /// 确认帖子解决
    pub async fn solved(
        &self,
        post_id: &str,
        input: &SolvedInput,
    ) -> Result<SolvedResponse, ApiError> {
        let path = format!("/posts/{}/solved", post_id);
        let response: SolvedResponse = self.post(&path, input).await?;
        Ok(response)
    }

    /// 发放悬赏
    pub async fn reward(
        &self,
        post_id: &str,
        input: &RewardInput,
    ) -> Result<serde_json::Value, ApiError> {
        let path = format!("/posts/{}/reward", post_id);
        let response: serde_json::Value = self.post(&path, input).await?;
        Ok(response)
    }

    /// 请作者喝咖啡
    pub async fn coffee(
        &self,
        post_id: &str,
        amount: Option<i64>,
    ) -> Result<CoffeeResponse, ApiError> {
        let input = CoffeeInput {
            post_id: post_id.to_string(),
            amount_cents: amount,
            currency: None,
        };
        let response: CoffeeResponse = self.post("/payments/coffee", &input).await?;
        Ok(response)
    }
}

impl Default for ApiClient {
    fn default() -> Self {
        Self::new().expect("Failed to create API client")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_error_display() {
        let err = ApiError::NotAuthenticated;
        assert_eq!(err.to_string(), "认证失败，请先登录");

        let err = ApiError::RequestFailed("connection refused".to_string());
        assert_eq!(err.to_string(), "请求失败: connection refused");

        let err = ApiError::ApiError("bad request".to_string());
        assert_eq!(err.to_string(), "API 返回错误: bad request");

        let err = ApiError::ParseError("invalid json".to_string());
        assert_eq!(err.to_string(), "响应解析失败: invalid json");

        let err = ApiError::ServerError("internal server error".to_string());
        assert_eq!(err.to_string(), "服务器错误: internal server error");

        let err = ApiError::Timeout;
        assert_eq!(err.to_string(), "请求超时");

        let err = ApiError::InvalidUrl("invalid url".to_string());
        assert_eq!(err.to_string(), "无效的 URL: invalid url");
    }

    #[test]
    fn test_api_error_from_json() {
        let json_err = serde_json::from_str::<serde_json::Value>("invalid").unwrap_err();
        let api_err: ApiError = json_err.into();
        assert!(matches!(api_err, ApiError::ParseError(_)));
    }

    #[test]
    fn test_api_client_with_token() {
        let config = Config {
            token: Some("test_token".to_string()),
            api_url: "https://api.test.com".to_string(),
            username: Some("testuser".to_string()),
        };

        let client = ApiClient::with_config(&config);
        assert!(client.has_token());
        assert_eq!(client.base_url(), "https://api.test.com");
    }

    #[test]
    fn test_api_client_without_token() {
        let config = Config::default();

        let client = ApiClient::with_config(&config);
        assert!(!client.has_token());
    }

    #[test]
    fn test_api_client_with_token_method() {
        let config = Config::default();
        let client = ApiClient::with_config(&config).with_token("my_token".to_string());
        assert!(client.has_token());
    }

    #[test]
    fn test_api_client_debug() {
        let config = Config::default();
        let client = ApiClient::with_config(&config);
        let debug_str = format!("{:?}", client);
        assert!(debug_str.contains("ApiClient"));
    }
}
