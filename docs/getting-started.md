# 快速入门

本指南将帮助你快速上手 zhan-cli 命令行工具。

## 第一次使用

### 1. 检查安装

```bash
zhan --version
```

如果显示版本号，说明安装成功。

### 2. 登录账号

有两种登录方式:

**方式一: 设备码登录 (推荐)**

```bash
zhan login
```

终端会显示一个 URL 和设备码:
1. 在浏览器中打开显示的 URL
2. 输入设备码
3. 确认授权

**方式二: API Token 登录**

如果你已经有 API Token:

```bash
zhan login --token YOUR_API_TOKEN
```

### 3. 验证登录

```bash
zhan whoami
```

应该显示你的用户名和头像信息。

## 日常使用流程

### 浏览社区

```bash
# 查看最新帖子
zhan feed

# 按类型筛选 (debug, code-review, config, question)
zhan feed --type question

# 指定返回数量
zhan feed --limit 20
```

### 搜索内容

```bash
# 搜索帖子
zhan search "Rust async"

# 搜索特定类型的帖子
zhan search "performance" --type debug
```

### 查看帖子

```bash
# 查看帖子详情
zhan view 12345

# 帖子 ID 可以从 feed 或搜索结果中获取
```

### 发布帖子

```bash
# 发布一个问题
zhan post \
  --title "如何优化 Rust 编译速度?" \
  --content "我的项目编译很慢，有什么优化建议吗?" \
  --type question

# 发布一个调试请求
zhan post \
  --title "程序崩溃求助" \
  --content "## 环境\nUbuntu 22.04, Rust 1.75\n\n## 问题\n程序运行时崩溃...\n\n## 错误日志\n..." \
  --type debug \
  --tags "rust,crash,debug"

# 发布带悬赏的帖子
zhan post \
  --title "需要性能优化" \
  --content "详情..." \
  --bounty 500  # 500 分 = 5 元
```

### 确认问题解决

当别人的帖子解决了你的问题时:

```bash
zhan solved --post-id 12345 --bounty 100 --time-saved 30
```

参数说明:
- `--post-id`: 解决你问题的帖子 ID
- `--bounty`: 悬赏金额 (分)
- `--time-saved`: 节省的时间 (分钟)

### 发放悬赏

```bash
zhan reward --post-id 12345 --bounty 200
```

### 查看个人统计

```bash
zhan stats
```

显示你的:
- 发帖数量
- 解决问题数
- 声望值
- 累计收益

### 请作者喝咖啡

```bash
zhan coffee
```

## 常用技巧

### 使用帖子模板

```bash
# 查看可用模板
zhan template list

# 生成 debug 模板
zhan template show debug

# 生成 question 模板
zhan template show question
```

### 检查 API 状态

如果遇到网络问题:

```bash
zhan health
```

### 退出登录

```bash
zhan logout
```

## 下一步

- 阅读 [命令参考](commands.md) 了解所有命令
- 查看 [使用示例](examples.md) 学习更多场景
- 查阅 [FAQ](faq.md) 解决常见问题