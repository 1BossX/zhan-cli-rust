use anyhow::Result;
use confy::ConfyError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// CLI 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// 访问令牌
    pub token: Option<String>,
    /// API 服务器地址
    #[serde(default = "default_api_url")]
    pub api_url: String,
    /// 用户名
    pub username: Option<String>,
}

fn default_api_url() -> String {
    "https://api.zhanjian.space".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            token: None,
            api_url: default_api_url(),
            username: None,
        }
    }
}

/// 配置错误
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("配置文件读取失败: {0}")]
    ReadError(String),
    #[error("配置文件写入失败: {0}")]
    WriteError(String),
    #[error("配置路径无效")]
    InvalidPath,
    #[error("令牌格式无效")]
    InvalidToken,
}

impl From<ConfyError> for ConfigError {
    fn from(e: ConfyError) -> Self {
        ConfigError::ReadError(e.to_string())
    }
}

impl Config {
    /// 获取配置文件路径
    pub fn path() -> PathBuf {
        // 直接使用备用路径
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("zhan-cli");
        path.push("config.toml");
        path
    }

    /// 加载配置
    pub fn load() -> Result<Self, ConfigError> {
        let config: Config = confy::load("zhan-cli", "config")?;
        Ok(config)
    }

    /// 保存配置
    pub fn save(&self) -> Result<(), ConfigError> {
        confy::store("zhan-cli", "config", self)?;
        Ok(())
    }

    /// 设置令牌
    pub fn set_token(&mut self, token: String) {
        if token.is_empty() {
            return;
        }
        self.token = Some(token);
    }

    /// 清除令牌（退出登录时使用）
    pub fn clear_token(&mut self) {
        self.token = None;
        self.username = None;
    }

    /// 检查是否已登录
    pub fn is_logged_in(&self) -> bool {
        self.token.is_some()
    }

    /// 验证令牌格式
    pub fn validate_token(&self) -> Result<(), ConfigError> {
        if let Some(ref token) = self.token {
            // 简单验证：令牌不应为空且长度应合理
            if token.is_empty() {
                return Err(ConfigError::InvalidToken);
            }
            if token.len() < 10 {
                return Err(ConfigError::InvalidToken);
            }
        }
        Ok(())
    }

    /// 获取 API URL（带协议）
    pub fn get_api_url(&self) -> &str {
        &self.api_url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.api_url, "https://api.zhanjian.space");
        assert!(config.token.is_none());
        assert!(config.username.is_none());
        assert!(!config.is_logged_in());
    }

    #[test]
    fn test_config_with_token() {
        let mut config = Config::default();
        config.set_token("test_token_123".to_string());
        assert!(config.is_logged_in());
        assert_eq!(config.token, Some("test_token_123".to_string()));
    }

    #[test]
    fn test_clear_token() {
        let mut config = Config::default();
        config.set_token("test_token_123".to_string());
        config.username = Some("testuser".to_string());

        config.clear_token();

        assert!(config.token.is_none());
        assert!(config.username.is_none());
        assert!(!config.is_logged_in());
    }

    #[test]
    fn test_set_empty_token() {
        let mut config = Config::default();
        config.set_token("".to_string());
        assert!(config.token.is_none());
    }

    #[test]
    fn test_validate_token_valid() {
        let mut config = Config::default();
        config.set_token("valid_token_12345".to_string());
        assert!(config.validate_token().is_ok());
    }

    #[test]
    fn test_validate_token_empty() {
        // set_token ignores empty strings, so token remains None
        let mut config = Config::default();
        config.set_token("".to_string());
        // Empty string is ignored, so validation passes (no token to validate)
        assert!(config.validate_token().is_ok());
    }

    #[test]
    fn test_validate_token_too_short() {
        let mut config = Config::default();
        config.set_token("short".to_string());
        assert!(matches!(
            config.validate_token(),
            Err(ConfigError::InvalidToken)
        ));
    }

    #[test]
    fn test_validate_token_none() {
        let config = Config::default();
        assert!(config.validate_token().is_ok());
    }

    #[test]
    fn test_config_error_display() {
        let err = ConfigError::ReadError("file not found".to_string());
        assert_eq!(err.to_string(), "配置文件读取失败: file not found");

        let err = ConfigError::WriteError("permission denied".to_string());
        assert_eq!(err.to_string(), "配置文件写入失败: permission denied");

        let err = ConfigError::InvalidPath;
        assert_eq!(err.to_string(), "配置路径无效");

        let err = ConfigError::InvalidToken;
        assert_eq!(err.to_string(), "令牌格式无效");
    }

    #[test]
    fn test_get_api_url() {
        let config = Config::default();
        assert_eq!(config.get_api_url(), "https://api.zhanjian.space");

        let config = Config {
            api_url: "https://custom.api.com".to_string(),
            ..Default::default()
        };
        assert_eq!(config.get_api_url(), "https://custom.api.com");
    }

    #[test]
    fn test_config_serialization() {
        let config = Config {
            token: Some("test_token".to_string()),
            api_url: "https://api.example.com".to_string(),
            username: Some("testuser".to_string()),
        };

        let serialized = serde_json::to_string(&config).unwrap();
        assert!(serialized.contains("test_token"));
        assert!(serialized.contains("api.example.com"));
        assert!(serialized.contains("testuser"));

        let deserialized: Config = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.token, Some("test_token".to_string()));
        assert_eq!(deserialized.api_url, "https://api.example.com");
        assert_eq!(deserialized.username, Some("testuser".to_string()));
    }

    #[test]
    fn test_default_api_url() {
        assert_eq!(default_api_url(), "https://api.zhanjian.space");
    }
}
