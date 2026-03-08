# Zhan CLI Rust 移植计划

将 Node.js/TypeScript 版本的 `kzhan-cli` 移植为 Rust 版本。

---

## 项目信息

- **原项目**: https://github.com/1BossX/kzhan-cli (Node.js/TypeScript)
- **Rust 仓库**: https://github.com/1BossX/zhan-cli-rust
- **目标**: 功能对等的 Rust CLI 工具

---

## 移植任务计划

### 阶段 1: 项目基础架构

| 任务 | 描述 | 预估时间 |
|------|------|----------|
| 1.1 | 创建 Cargo 项目结构 | 30min |
| 1.2 | 配置 CI/CD (GitHub Actions) | 1h |
| 1.3 | 添加依赖: clap, reqwest, serde, tokio, confy | 30min |

**依赖清单**:
- `clap` - 命令行参数解析
- `reqwest` - HTTP 客户端
- `serde` + `serde_json` - JSON 序列化/反序列化
- `tokio` - 异步运行时
- `confy` - 配置文件管理
- `anyhow` / `thiserror` - 错误处理
- `colored` / `owo-colors` - 终端颜色输出
- `indicatif` - 进度条/加载动画

---

### 阶段 2: 核心模块实现

| 任务 | 描述 | 对应原文件 |
|------|------|-----------|
| 2.1 | Config 模块 - 配置管理 | `lib/config.ts` |
| 2.2 | API Client 模块 - HTTP 请求 + 重试机制 | `lib/api.ts` |
| 2.3 | Auth 模块 - 设备码登录流程 | `commands/login.ts` |

**Config 模块设计**:
```rust
// src/config.rs
pub struct Config {
    pub token: Option<String>,
    pub api_url: String,
    pub username: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError>;
    pub fn save(&self) -> Result<(), ConfigError>;
    pub fn path() -> PathBuf;
}
```

**API Client 模块设计**:
```rust
// src/api.rs
pub struct ApiClient {
    client: reqwest::Client,
    base_url: String,
    token: Option<String>,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self;
    pub fn with_token(self, token: String) -> Self;
    pub async fn get(&self, path: &str) -> Result<Response>;
    pub async fn post(&self, path: &str, body: Value) -> Result<Response>;
    pub async fn poll_device_token(&self, device_code: &str) -> Result<String>;
}
```

---

### 阶段 3: 命令实现

| 任务 | 命令 | 优先级 | 说明 |
|------|------|--------|------|
| 3.1 | `health` | P0 | API 健康检查 |
| 3.2 | `config` | P0 | 配置管理 (get/set/reset/path) |
| 3.3 | `login` | P0 | 设备码登录 |
| 3.4 | `whoami` | P0 | 显示用户信息 |
| 3.5 | `logout` | P0 | 退出登录 |
| 3.6 | `feed` | P1 | 浏览社区 Feed |
| 3.7 | `search` | P1 | 搜索帖子 |
| 3.8 | `view` | P1 | 查看帖子详情 |
| 3.9 | `post` | P2 | 发布技术内容 |
| 3.10 | `coffee`/`solved`/`reward` | P2 | 互动功能 |

---

### 阶段 4: 测试与优化

| 任务 | 描述 |
|------|------|
| 4.1 | 单元测试 (API Client, Config) |
| 4.2 | 集成测试 |
| 4.3 | 性能优化 |
| 4.4 | 文档更新 |

---

## 设备码登录流程 (OAuth 2.0 Device Flow)

```
┌─────────┐                                    ┌─────────────┐
│  kzhan  │ ──(1) POST /auth/device/start────▶ │   Server    │
│   CLI   │                                    │             │
│         │ ◀────────deviceCode + userCode──── │             │
│         │                                    │             │
│         │ ──(2) 显示验证码给用户────────────▶ │             │
│         │                                    │             │
│         │ ◀────────用户在浏览器确认───────── │             │
│         │                                    │             │
│         │ ──(3) POST /auth/device/poll ────▶ │             │
│         │    (每 N 秒轮询一次)                │             │
│         │                                    │             │
│         │ ◀────────────token──────────────── │             │
└─────────┘                                    └─────────────┘
```

**具体步骤**:

| 步骤 | 动作 | 说明 |
|------|------|------|
| 1 | CLI → Server | POST /auth/device/start，请求开始设备登录 |
| 2 | Server → CLI | 返回 deviceCode（内部用）+ userCode（给用户看）+ verificationUri |
| 3 | CLI 显示 | 在终端打印验证码和验证地址 |
| 4 | 用户操作 | 浏览器打开验证地址，输入验证码确认 |
| 5 | CLI 轮询 | POST /auth/device/poll，每隔几秒问 Server："用户确认了吗？" |
| 6 | Server → CLI | 用户确认后，返回 token |

**错误处理**:
- `authorization_pending` — 继续轮询
- `slow_down` — 降低轮询频率
- `expired_token` — 验证码过期
- `access_denied` — 用户拒绝
- `already_used` — 验证码已被使用

---

## 配置文件

**存储位置**:
- macOS: `~/Library/Application Support/zhan-cli/config.toml`
- Linux: `~/.config/zhan-cli/config.toml`

**配置内容**:
```toml
token = "jwt_xxx"
api_url = "https://api.zhanjian.space"
username = "alice"
```

---

## API 端点

| 端点 | 方法 | 描述 |
|------|------|------|
| `/health` | GET | 健康检查 |
| `/auth/device/start` | POST | 开始设备码登录 |
| `/auth/device/poll` | POST | 轮询设备码状态 |
| `/auth/device/approve` | POST | 浏览器端确认授权 |
| `/me` | GET | 获取当前用户信息 |
| `/feed` | GET | 获取社区 Feed |
| `/search` | GET | 搜索帖子 |
| `/posts/{id}` | GET | 获取帖子详情 |
| `/posts` | POST | 发布帖子 |

---

## 参考实现

- **原项目**: `~/.openclaw/workspace/kzhan-cli/`
- **关键文件**:
  - `packages/cli/src/commands/login.ts` - 登录逻辑
  - `packages/cli/src/lib/config.ts` - 配置管理
  - `packages/cli/src/lib/api.ts` - API 客户端

---

## 下一步行动

1. ✅ 创建空的 Rust 仓库
2. [ ] 初始化 Cargo 项目
3. [ ] 添加基础依赖
4. [ ] 实现 Config 模块
5. [ ] 实现 API Client 模块
6. [ ] 实现 `health` 命令
7. [ ] 实现 `login` 命令

---

*创建时间: 2026-03-08*
*来源: Discord 对话移植计划*
