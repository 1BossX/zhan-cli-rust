# zhan-cli

🏃‍♂️ 栈间 (Zhanjian) CLI - 命令行工具

> 栈间 (Zhanjian) 是一个开发者社区，专注于问题解答、技术分享和知识沉淀。

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Version](https://img.shields.io/github/v/release/1BossX/zhan-cli-rust)](https://github.com/1BossX/zhan-cli-rust/releases)

## 什么是栈间？

**栈间** 是一个新一代开发者社区，致力于打造一个高效、友好的技术交流平台。

### 核心特色

- 💬 **问答社区** - 技术问题快速获取解答
- 🎁 **悬赏系统** - 用真金白银激励优质回答
- 📚 **知识沉淀** - 好的问题和答案值得被收藏
- ☕ **打赏机制** - 感谢作者的付出

### Web 端功能

访问 [zhan.io](https://zhan.io) 使用完整功能：

| 功能 | 说明 |
|------|------|
| 浏览 Feed | 查看最新、最热门的帖子 |
| 搜索 | 关键词搜索技术内容 |
| 提问 | 发布问题寻求帮助 |
| 回答 | 解答问题，获得悬赏 |
| 采纳 | 提问者采纳最佳答案 |
| 打赏 | 给优质作者送咖啡 |
| 个人主页 | 查看个人积分、统计 |

### CLI 特色

`zhan-cli` 是栈间的命令行客户端，让你可以在终端完成大部分操作：

- 🔐 **登录** - 设备码登录 / API Token 登录
- 📰 **浏览** - 查看社区 Feed、搜索帖子
- 📝 **发帖** - 发布问题、技术分享
- 🎁 **悬赏** - 确认解决、发放悬赏
- 📊 **统计** - 查看个人数据
- ☕ **打赏** - 请作者喝咖啡

## 安装

### 一键安装 (推荐)

```bash
# macOS / Linux
curl -sSL https://raw.githubusercontent.com/1BossX/zhan-cli-rust/main/install.sh | sh
```

如果遇到权限问题，安装脚本会自动安装到 `~/.local/bin` 并帮你添加到 PATH。

### 从 Release 下载

访问 [Releases](https://github.com/1BossX/zhan-cli-rust/releases) 下载预编译的二进制：

| 平台 | 下载 |
|------|------|
| macOS Apple Silicon | `zhan-v*-aarch64-apple-darwin.tar.gz` |
| macOS Intel | `zhan-v*-x86_64-apple-darwin.tar.gz` |
| Linux x86_64 | `zhan-v*-x86_64-unknown-linux-gnu.tar.gz` |

### 从源码编译

```bash
cargo install --package zhan --locked
```

## 快速开始

### 登录

```bash
# 设备码登录 (推荐)
zhan login

# 或使用 API Token
zhan login --token YOUR_API_TOKEN
```

### 常用命令

```bash
# 查看当前用户
zhan whoami

# 浏览社区 Feed
zhan feed                    # 最新帖子
zhan feed --type question    # 只看提问

# 搜索帖子
zhan search "Rust"

# 查看帖子详情
zhan view <post_id>

# 发布帖子
zhan post --title "问题标题" --content "详细内容..." --type question

# 确认问题解决并发放悬赏
zhan solved --post-id <id> --bounty 100 --time-saved 30

# 查看个人统计
zhan stats

# 请作者喝咖啡
zhan coffee --post-id <id> --amount 100
```

### 命令列表

| 命令 | 简写 | 说明 |
|------|------|------|
| `login` | `l` | 登录 |
| `whoami` | `w` | 当前用户信息 |
| `logout` | `lo` | 退出登录 |
| `health` | `h` | 检查 API 状态 |
| `feed` | `f` | 浏览社区 Feed |
| `search` | `s` | 搜索帖子 |
| `view` | `v` | 查看帖子详情 |
| `post` | `p` | 发布新帖子 |
| `solved` | | 确认帖子解决 |
| `reward` | | 发放悬赏 |
| `stats` | | 个人统计 |
| `coffee` | | 请作者喝咖啡 |
| `template` | `t` | 模板管理 |

## 开发

```bash
# 克隆项目
git clone https://github.com/1BossX/zhan-cli-rust.git
cd zhan-cli-rust

# 构建
cargo build --release

# 测试
cargo test --workspace
```

## 项目结构

```
zhan-cli-rust/
├── packages/
│   ├── zhan-sdk/       # API SDK
│   │   └── src/
│   │       ├── auth.rs     # 认证模块
│   │       ├── client.rs   # API 客户端
│   │       ├── config.rs   # 配置管理
│   │       └── types.rs    # 类型定义
│   └── zhan/           # CLI 主程序
│       └── src/
│           ├── main.rs     # 入口
│           └── commands/   # 命令实现
├── .github/
│   └── workflows/      # CI/CD
├── install.sh          # 一键安装脚本
├── README.md           # 项目文档
├── SDK.md              # SDK 文档
├── API.md              # API 文档
└── CHANGELOG.md        # 更新日志
```

## 路线图

### v0.1.x - Alpha 阶段

- [x] 基础登录 (设备码 / Token)
- [x] Feed 浏览
- [x] 帖子搜索、查看、发布
- [x] 悬赏系统
- [x] 个人统计
- [x] 多平台支持 (macOS, Linux)

### v0.2.0 - Beta 阶段

- [ ] 评论功能
- [ ] 消息通知
- [ ] 收藏功能
- [ ] 更多模板类型
- [ ] 交互式 TUI

### v1.0.0 - 正式版

- [ ] 完整的账号系统
- [ ] 积分排行榜
- [ ] 社区徽章
- [ ] API 稳定性保证

### 未来展望

- 🌐 **WebAssembly** - 浏览器端运行
- 📱 **移动端** - iOS/Android 原生应用
- 🔌 **IDE 插件** - VS Code、JetBrains 集成
- 🤖 **AI 助手** - 结合大语言模型的智能问答

## 相关链接

- 🌐 **官网**: [https://zhan.io](https://zhan.io)
- 💬 **社区**: [https://zhan.io/feed](https://zhan.io/feed)
- 📖 **文档**: 
  - [README](README.md)
  - [SDK](SDK.md)
  - [API](API.md)
  - [CHANGELOG](CHANGELOG.md)
- 🐛 **反馈**: [Issues](https://github.com/1BossX/zhan-cli-rust/issues)

## License

MIT
