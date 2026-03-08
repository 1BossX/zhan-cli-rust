# zhan-cli

🏃‍♂️ 栈间 (Zhanjian) CLI - 命令行工具

## 功能特性

- 🔐 **设备码登录** - 支持 API Token 登录
- 📰 **社区浏览** - 浏览社区 Feed，搜索帖子
- 📝 **帖子管理** - 发布、查看帖子
- 🎁 **悬赏系统** - 确认问题解决、发放悬赏
- 📊 **个人统计** - 查看个人数据
- ☕ **打赏功能** - 请作者喝咖啡

## 安装

### 一键安装 (推荐)

```bash
# macOS / Linux
curl -sSL https://raw.githubusercontent.com/1BossX/zhan-cli-rust/main/install.sh | sh
```

### 从 Release 下载

访问 [Releases](https://github.com/1BossX/zhan-cli-rust/releases) 下载预编译的二进制文件：

| 平台 | 下载 |
|------|------|
| Linux x86_64 | zhan-x86_64-unknown-linux-gnu.tar.gz |
| Linux ARM64 | zhan-aarch64-unknown-linux-gnu.tar.gz |
| macOS x86_64 | zhan-x86_64-apple-darwin.tar.gz |
| Windows x86_64 | zhan-x86_64-pc-windows-gnu.zip |

### 从源码编译

```bash
cargo install --package zhan --locked
```

## 使用方法

### 登录

```bash
# 设备码登录
zhan login

# 或使用 API Token
zhan login --token YOUR_API_TOKEN
```

### 查看当前用户

```bash
zhan whoami
```

### 浏览社区

```bash
# 查看 Feed
zhan feed

# 搜索帖子
zhan search "Rust"
```

### 发布帖子

```bash
zhan post --title "问题标题" --content "问题描述" --type question
```

### 确认解决

```bash
zhan solved --post-id <ID> --bounty 100 --time-saved 30
```

## 命令列表

| 命令 | 说明 |
|------|------|
| `login` | 登录 |
| `whoami` | 当前用户信息 |
| `logout` | 退出登录 |
| `health` | 检查 API 状态 |
| `feed` | 浏览社区 Feed |
| `search` | 搜索帖子 |
| `view` | 查看帖子详情 |
| `post` | 发布新帖子 |
| `solved` | 确认帖子解决 |
| `reward` | 发放悬赏 |
| `stats` | 个人统计 |
| `coffee` | 请作者喝咖啡 |
| `template` | 模板管理 |

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

## License

MIT
