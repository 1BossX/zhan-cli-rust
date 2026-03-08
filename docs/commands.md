# 命令参考

本文档详细介绍 zhan-cli 的所有命令和选项。

## 认证命令

### login - 登录

```bash
# 设备码登录
zhan login

# 使用 API Token 登录
zhan login --token <TOKEN>
```

**选项:**
| 选项 | 说明 |
|------|------|
| `--token` | 直接使用 API Token 登录 |

**说明:** 设备码登录会显示一个 URL 和设备码，需要在浏览器中完成授权。API Token 登录适合自动化脚本或 CI/CD 环境。

---

### whoami - 当前用户

```bash
zhan whoami
```

显示当前登录用户的:
- 用户名
- 头像
- 声望值
- 注册时间

---

### logout - 退出登录

```bash
zhan logout
```

清除本地保存的 Token。

---

## 信息命令

### health - API 状态

```bash
zhan health
```

检查与栈间 API 服务器的连接状态。

**返回:**
- ✅ 连接正常
- ❌ 连接失败 (显示错误信息)

---

### feed - 浏览社区

```bash
# 查看最新帖子
zhan feed

# 按类型筛选
zhan feed --type <TYPE>

# 指定返回数量
zhan feed --limit <NUM>
```

**选项:**
| 选项 | 说明 | 默认值 |
|------|------|--------|
| `--type` | 帖子类型 (debug/code-review/config/question) | 全部 |
| `--limit` | 返回数量 (1-100) | 20 |

---

### search - 搜索帖子

```bash
zhan search <QUERY>

# 搜索并指定类型
zhan search <QUERY> --type <TYPE>

# 限制结果数量
zhan search <QUERY> --limit <NUM>
```

**参数:**
| 参数 | 说明 |
|------|------|
| `<QUERY>` | 搜索关键词 |

**选项:**
| 选项 | 说明 |
|------|------|
| `--type` | 帖子类型过滤 |
| `--limit` | 返回数量 |

---

### view - 查看帖子

```bash
zhan view <POST_ID>
```

**参数:**
| 参数 | 说明 |
|------|------|
| `<POST_ID>` | 帖子 ID |

**显示信息:**
- 标题和内容
- 作者信息
- 类型和标签
- 浏览/点赞/评论数
- 悬赏金额
- 发布时间

---

### stats - 个人统计

```bash
zhan stats
```

显示你的个人数据:
- 总发帖数
- 已解决数
- 平均 CV 数
- 声望值
- 账户余额

---

## 内容命令

### post - 发布帖子

```bash
zhan post \
  --title "<标题>" \
  --content "<内容>" \
  --type <TYPE> \
  [--tags <TAGS>] \
  [--bounty <AMOUNT>]
```

**选项:**
| 选项 | 说明 | 必需 |
|------|------|------|
| `--title` | 帖子标题 | ✅ |
| `--content` | 帖子内容 (Markdown) | ✅ |
| `--type` | 帖子类型 | ✅ |
| `--tags` | 标签 (逗号分隔) | ❌ |
| `--bounty` | 悬赏金额 (分) | ❌ |

**类型说明:**
| 类型 | 说明 | 适用场景 |
|------|------|----------|
| `debug` | 调试请求 | 需要帮忙调试代码 |
| `code-review` | 代码审查 | 需要代码审查建议 |
| `config` | 配置问题 | 环境配置问题 |
| `question` | 问题咨询 | 一般性问题 |

**示例:**

```bash
# 发布简单问题
zhan post \
  --title "如何正确使用 async?" \
  --content "我不太理解 Rust 的 async 机制..." \
  --type question

# 发布调试请求
zhan post \
  --title "程序崩溃，请帮忙看看" \
  --content "## 环境\nUbuntu 22.04\n\n## 代码\n\`\`\`rust\nfn main() {\n    panic!(\"test\");\n}\n\`\`\`\n\n## 错误\nthread 'main' panicked at 'test'" \
  --type debug \
  --tags "rust,panic"

# 发布带悬赏的问题
zhan post \
  --title "性能优化求助" \
  --content "详情..." \
  --type question \
  --bounty 500
```

---

### solved - 确认解决

```bash
zhan solved --post-id <ID> --bounty <AMOUNT> --time-saved <MINUTES>
```

**选项:**
| 选项 | 说明 | 必需 |
|------|------|------|
| `--post-id` | 解决你问题的帖子 ID | ✅ |
| `--bounty` | 支付悬赏金额 (分) | ✅ |
| `--time-saved` | 节省的时间 (分钟) | ✅ |

**说明:** 当你在社区提问，有人给出了有效的解决方案，可以使用此命令确认并发放悬赏。

---

### reward - 发放悬赏

```bash
zhan reward --post-id <ID> --bounty <AMOUNT>
```

**选项:**
| 选项 | 说明 | 必需 |
|------|------|------|
| `--post-id` | 帖子 ID | ✅ |
| `--bounty` | 悬赏金额 (分) | ✅ |

**说明:** 直接向指定帖子作者发放悬赏，不需要先标记为"已解决"。

---

## 娱乐命令

### coffee - 请作者喝咖啡

```bash
zhan coffee

# 指定金额
zhan coffee --amount 10
```

**选项:**
| 选项 | 说明 | 默认值 |
|------|------|--------|
| `--amount` | 金额 (元) | 5 |

**说明:** 捐赠一点钱支持开发者继续维护这个项目 🥤

---

## 工具命令

### template - 帖子模板

```bash
# 列出所有模板
zhan template list

# 显示模板内容
zhan template show <TYPE>
```

**模板类型:**
- `debug` - 调试请求模板
- `question` - 问题咨询模板
- `config` - 配置问题模板
- `code-review` - 代码审查模板

**示例:**

```bash
# 查看 debug 模板
zhan template show debug

# 复制到剪贴板 (macOS)
zhan template show debug | pbcopy

# 复制到剪贴板 (Linux)
zhan template show debug | xclip
```

---

## 全局选项

所有命令都支持以下全局选项:

```bash
--help     # 显示帮助信息
--version  # 显示版本号
--verbose  # 显示详细日志 (可选)
```

**示例:**

```bash
zhan --help
zhan --version
zhan post --help
```