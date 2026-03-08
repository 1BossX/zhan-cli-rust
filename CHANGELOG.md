# Changelog

All notable changes to this project will be documented in this file.

## [0.1.0-alpha.1] - 2026-03-08

### Added
- 设备码登录 (Device login)
- API Token 登录
- 查看当前用户信息 (whoami)
- 退出登录 (logout)
- API 连接状态检查 (health)
- 社区 Feed 浏览
- 帖子搜索
- 帖子详情查看
- 发布新帖子 (支持 Markdown)
- 悬赏确认 (solved)
- 悬赏发放 (reward)
- 个人统计 (stats)
- 请作者喝咖啡 (coffee)
- 模板管理

### Technical
- Rust workspace 项目结构
- 使用 tokio 异步运行时
- 使用 reqwest + rustls (无 OpenSSL 依赖)
- CI/CD 自动化 (GitHub Actions)
- 多平台发布支持 (Linux, macOS, Windows)
