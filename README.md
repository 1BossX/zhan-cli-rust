# zhan-cli

🏃‍♂️ 栈间 (Zhanjian) CLI - 命令行工具

> 栈间社区命令行客户端，让你更高效地浏览社区、发布帖子、管理悬赏

## 快速开始

```bash
# 1. 安装 (详见 docs/installation.md)
curl -sSL https://raw.githubusercontent.com/1BossX/zhan-cli-rust/main/install.sh | sh

# 2. 登录
zhan login

# 3. 查看帮助
zhan --help
```

## 功能特性

| 功能 | 说明 |
|------|------|
| 🔐 **认证登录** | 设备码登录、API Token 登录 |
| 📰 **社区浏览** | 浏览 Feed、搜索帖子、查看详情 |
| 📝 **帖子管理** | 发布帖子（debug/code-review/config/question） |
| 🎁 **悬赏系统** | 确认问题解决、发放悬赏奖励 |
| 📊 **数据统计** | 查看个人贡献统计 |
| ☕ **打赏作者** | 请作者喝咖啡 |

## 文档目录

| 文档 | 说明 |
|------|------|
| [安装指南](docs/installation.md) | 各种安装方式的详细说明 |
| [快速入门](docs/getting-started.md) | 新手入门必看 |
| [命令参考](docs/commands.md) | 所有命令详解 |
| [使用示例](docs/examples.md) | 实际使用场景示例 |
| [FAQ](docs/faq.md) | 常见问题解答 |

## 命令一览

```
zhan login              # 登录 (设备码)
zhan login --token XXX  # 使用 Token 登录
zhan whoami             # 查看当前用户
zhan logout             # 退出登录
zhan health             # 检查 API 状态
zhan feed               # 浏览社区 Feed
zhan search "关键词"    # 搜索帖子
zhan view <ID>          # 查看帖子详情
zhan post --title ...   # 发布新帖子
zhan solved --post-id X # 确认问题解决
zhan reward --post-id X # 发放悬赏
zhan stats              # 查看个人统计
zhan coffee             # 请作者喝咖啡
zhan template           # 帖子模板管理
```

## 开发

```bash
# 克隆项目
git clone https://github.com/1BossX/zhan-cli-rust.git
cd zhan-cli-rust

# 构建
cargo build --release

# 运行
cargo run -- --help

# 测试
cargo test --workspace
```

## 相关链接

- 🌐 栈间官网: https://zhanjian.com
- 📖 在线文档: https://docs.zhanjian.com
- 🐛 问题反馈: https://github.com/1BossX/zhan-cli-rust/issues
- 💬 社区讨论: https://discord.gg/zhanjian

## License

MIT