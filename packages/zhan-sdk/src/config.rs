use anyhow::{Context, Result};
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.api_url, "https://api.zhanjian.space");
        assert!(config.token.is_none());
    }
}
