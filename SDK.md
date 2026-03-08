# zhan-sdk

🏃‍♂️ 栈间 Zhanjian SDK for Rust

## 概述

`zhan-sdk` 是一个用于与[栈间](https://zhan.io) API 交互的 Rust SDK。

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
zhan-sdk = { version = "0.1", features = ["derive"] }
```

## 快速开始

### 创建客户端

```rust
use zhan_sdk::ApiClient;

// 从默认配置加载
let client = ApiClient::new()?;

// 或使用自定义 token
let client = ApiClient::new()?.with_token("your-token");
```

### 设备码登录

```rust
use zhan_sdk::{ApiClient, DeviceLogin};

// 创建设备码登录
let login = DeviceLogin::new()?;
let device_code = login.start().await?;

// 用户扫码后轮询登录结果
let token = login.poll().await?;
```

## API 参考

### 认证

```rust
// 设备码登录
let login = DeviceLogin::new()?;
let response = login.start().await?;
println!("请访问: {}", response.verification_uri);
println!("输入验证码: {}", response.user_code);

// 轮询等待用户授权
let token = login.poll().await?;
```

### 用户

```rust
// 获取当前用户
let user = client.get_current_user().await?;
println!("用户名: {}", user.username);
```

### Feed

```rust
// 获取社区 Feed
let posts = client.get_feed(Some("question"), Some(20)).await?;
for post in posts {
    println!("{}", post.title);
}
```

### 搜索

```rust
// 搜索帖子
let results = client.search("Rust", None, None, Some(10)).await?;
```

### 帖子

```rust
use zhan_sdk::{ApiClient, CreatePostInput, PostType};

// 创建帖子
let input = CreatePostInput {
    title: "如何学习 Rust?".to_string(),
    content_md: "详细内容...".to_string(),
    post_type: PostType::Question,
    tags: Some(vec!["rust".to_string()]),
    bounty_cents: Some(100), // 悬赏 1 元
};
let result = client.create_post(&input).await?;
```

### 悬赏

```rust
use zhan_sdk::{RewardInput, SolvedInput};

// 确认解决
let solved = client.solved("post-id", &SolvedInput {
    bounty_cents: Some(100),
    time_saved_minutes: Some(30),
}).await?;

// 发放悬赏
let reward = client.reward("post-id", &RewardInput {
    answerer_user_id: "user-id".to_string(),
}).await?;
```

### 打赏

```rust
// 请作者喝咖啡
let coffee = client.coffee("post-id", Some(100)).await?;
// 返回 checkout_url 用于支付
```

### 统计

```rust
// 获取用户统计
let stats = client.get_stats().await?;
println!("帖子数: {}", stats.post_count);
println!("平均 CVS: {:.2}", stats.avg_cvs);
```

## 类型参考

### 核心类型

| 类型 | 说明 |
|------|------|
| `ApiClient` | API 客户端 |
| `Config` | 配置管理 |
| `DeviceLogin` | 设备码登录 |
| `User` | 用户信息 |
| `Post` | 帖子 |
| `UserStats` | 用户统计 |

### 枚举

| 类型 | 值 |
|------|-----|
| `PostType` | `Debug`, `CodeReview`, `Config`, `Question` |

## 错误处理

```rust
use zhan_sdk::ApiClientError;

match client.get_current_user().await {
    Ok(user) => println!("{}", user.username),
    Err(ApiClientError::NotAuthenticated) => {
        println!("请先登录!");
    }
    Err(e) => {
        println!("错误: {}", e);
    }
}
```

## 配置

SDK 会自动从以下位置加载配置：
- `~/.config/zhan/config.toml` (Linux)
- `~/Library/Application Support/zhan/config.toml` (macOS)
- `%APPDATA%\zhan\config.toml` (Windows)

配置格式：

```toml
api_url = "https://api.zhan.io"
token = "your-access-token"
```
