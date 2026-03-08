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
}

/// API 客户端
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
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()?,
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
                return Err(ApiError::ApiError(format!("JSON 解析错误: {}", e)));
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
        let response: ApiResponse<User> = self.get("/users/me").await?;
        response.data.ok_or(ApiError::ApiError("获取用户信息失败".to_string()))
    }

    /// 获取 Feed
    pub async fn get_feed(&self, r#type: Option<&str>, limit: Option<u32>) -> Result<Vec<Post>, ApiError> {
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
    pub async fn search(&self, query: &str, r#type: Option<&str>, since: Option<u32>, limit: Option<u32>) -> Result<SearchResult, ApiError> {
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
}

impl Default for ApiClient {
    fn default() -> Self {
        Self::new().expect("Failed to create API client")
    }
}