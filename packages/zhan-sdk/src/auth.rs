use crate::client::{ApiClient, ApiError};
use crate::config::Config;
use anyhow::{Context, Result};
use std::time::Duration;

/// 登录结果
#[derive(Debug)]
pub struct LoginResult {
    pub token: String,
    pub username: String,
}

/// 设备码登录错误
#[derive(Debug, thiserror::Error)]
pub enum LoginError {
    #[error("启动设备登录失败: {0}")]
    StartFailed(String),
    #[error("登录超时")]
    Timeout,
    #[error("用户拒绝")]
    Denied,
    #[error("设备码已过期")]
    Expired,
    #[error("登录失败: {0}")]
    Other(String),
}

impl From<anyhow::Error> for LoginError {
    fn from(e: anyhow::Error) -> Self {
        LoginError::Other(e.to_string())
    }
}

/// 设备码登录
pub struct DeviceLogin {
    #[allow(dead_code)]
    client: ApiClient,
}

impl DeviceLogin {
    pub fn new() -> Result<Self> {
        Ok(Self {
            client: ApiClient::new()?,
        })
    }

    /// 开始设备码登录流程
    pub async fn start(&self) -> Result<DeviceCodeResponse, LoginError> {
        let config = Config::load().context("加载配置失败")?;
        let client = ApiClient::with_config(&config);

        let response: serde_json::Value = client
            .post(
                "/auth/device/start",
                &serde_json::json!({
                    "clientName": "zhan-cli-rust",
                    "scope": "cli:login"
                }),
            )
            .await
            .map_err(|e| LoginError::StartFailed(e.to_string()))?;

        // 解析响应
        let device_code = response["deviceCode"]
            .as_str()
            .context("缺少 deviceCode")?
            .to_string();
        let user_code = response["userCode"]
            .as_str()
            .context("缺少 userCode")?
            .to_string();
        let verification_uri = response["verificationUri"]
            .as_str()
            .context("缺少 verificationUri")?
            .to_string();
        let verification_uri_complete = response["verificationUriComplete"]
            .as_str()
            .map(|s| s.to_string());
        let expires_in = response["expiresIn"].as_u64().unwrap_or(300);
        let interval = response["interval"].as_u64().unwrap_or(5);

        Ok(DeviceCodeResponse {
            device_code,
            user_code,
            verification_uri,
            verification_uri_complete,
            expires_in,
            interval,
        })
    }

    /// 轮询等待用户确认
    pub async fn poll(&self, device_code: &str) -> Result<LoginResult, LoginError> {
        let config = Config::load().context("加载配置失败")?;
        let client = ApiClient::with_config(&config);

        // 默认超时 5 分钟
        let deadline = std::time::Instant::now() + Duration::from_secs(300);
        let mut interval_secs = 5;

        while std::time::Instant::now() < deadline {
            tokio::time::sleep(Duration::from_secs(interval_secs)).await;

            let result: Result<serde_json::Value, ApiError> = client
                .post(
                    "/auth/device/poll",
                    &serde_json::json!({ "deviceCode": device_code }),
                )
                .await;

            match result {
                Ok(response) => {
                    // 成功获取 token
                    // API 返回 {"token": "...", "user": {...}}
                    let token = response["token"]
                        .as_str()
                        .context("缺少 access_token")?
                        .to_string();

                    // 获取用户信息 (从 user 字段或 API)
                    let username = if let Some(user_obj) = response["user"].as_object() {
                        user_obj
                            .get("username")
                            .and_then(|u| u.as_str())
                            .unwrap_or("unknown")
                            .to_string()
                    } else {
                        let user_response: serde_json::Value = client
                            .get("/users/me")
                            .await
                            .map_err(|e| LoginError::Other(e.to_string()))?;
                        user_response["data"]["username"]
                            .as_str()
                            .unwrap_or("unknown")
                            .to_string()
                    };

                    // 保存 token 到配置
                    let mut config = Config::load().context("加载配置失败")?;
                    config.set_token(token.clone());
                    config.username = Some(username.clone());
                    config.save().context("保存配置失败")?;

                    return Ok(LoginResult { token, username });
                }
                Err(ApiError::ApiError(error_text)) => {
                    // 解析错误
                    if error_text.contains("authorization_pending") {
                        // 继续轮询
                        continue;
                    } else if error_text.contains("slow_down") {
                        // 降低轮询频率
                        interval_secs += 1;
                        continue;
                    } else if error_text.contains("expired_token") {
                        return Err(LoginError::Expired);
                    } else if error_text.contains("access_denied") {
                        return Err(LoginError::Denied);
                    } else {
                        return Err(LoginError::Other(error_text));
                    }
                }
                Err(e) => {
                    return Err(LoginError::Other(e.to_string()));
                }
            }
        }

        Err(LoginError::Timeout)
    }

    /// 使用 token 直接登录
    pub async fn login_with_token(&self, token: &str) -> Result<LoginResult, LoginError> {
        let config = Config::load().context("加载配置失败")?;
        let client = ApiClient::with_config(&config);

        // 使用新的 token 验证接口
        let response: serde_json::Value = client
            .post(
                "/auth/token/validate",
                &serde_json::json!({
                    "token": token
                }),
            )
            .await
            .map_err(|e| LoginError::Other(e.to_string()))?;

        let username = response["data"]["user"]["username"]
            .as_str()
            .context("无效的响应")?
            .to_string();

        // 保存 token 到配置
        let mut config = Config::load().context("加载配置失败")?;
        config.set_token(token.to_string());
        config.username = Some(username.clone());
        config.save().context("保存配置失败")?;

        Ok(LoginResult {
            token: token.to_string(),
            username,
        })
    }
}

/// 设备码响应
#[derive(Debug, serde::Deserialize)]
pub struct DeviceCodeResponse {
    #[serde(rename = "deviceCode")]
    pub device_code: String,
    #[serde(rename = "userCode")]
    pub user_code: String,
    #[serde(rename = "verificationUri")]
    pub verification_uri: String,
    #[serde(rename = "verificationUriComplete")]
    pub verification_uri_complete: Option<String>,
    #[serde(rename = "expiresIn")]
    pub expires_in: u64,
    pub interval: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login_result_default() {
        let result = LoginResult {
            token: "test_token".to_string(),
            username: "test_user".to_string(),
        };
        assert_eq!(result.token, "test_token");
        assert_eq!(result.username, "test_user");
    }

    #[test]
    fn test_device_code_response_default() {
        let response = DeviceCodeResponse {
            device_code: "device_123".to_string(),
            user_code: "user_456".to_string(),
            verification_uri: "https://example.com/verify".to_string(),
            verification_uri_complete: Some("https://example.com/verify/complete".to_string()),
            expires_in: 300,
            interval: 5,
        };
        assert_eq!(response.device_code, "device_123");
        assert_eq!(response.user_code, "user_456");
        assert!(response.verification_uri_complete.is_some());
        assert_eq!(response.expires_in, 300);
        assert_eq!(response.interval, 5);
    }

    #[test]
    fn test_login_error_display() {
        let err = LoginError::Timeout;
        assert_eq!(err.to_string(), "登录超时");

        let err = LoginError::Denied;
        assert_eq!(err.to_string(), "用户拒绝");

        let err = LoginError::Expired;
        assert_eq!(err.to_string(), "设备码已过期");

        let err = LoginError::StartFailed("network error".to_string());
        assert_eq!(err.to_string(), "启动设备登录失败: network error");

        let err = LoginError::Other("unknown error".to_string());
        assert_eq!(err.to_string(), "登录失败: unknown error");
    }

    #[test]
    fn test_login_error_from_anyhow() {
        let anyhow_err = anyhow::anyhow!("some error");
        let login_err: LoginError = anyhow_err.into();
        assert!(matches!(login_err, LoginError::Other(_)));
    }
}
