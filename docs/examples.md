# 使用示例

本文档提供真实的使用场景示例，帮助你更好地使用 zhan-cli。

## 场景一：日常浏览

### 快速浏览今日热门

```bash
# 查看最新的 10 条帖子
zhan feed --limit 10

# 只看调试类型的帖子
zhan feed --type debug --limit 10

# 只看有悬赏的帖子
zhan feed | grep -i bounty
```

### 关注特定话题

```bash
# 搜索 Rust 相关帖子
zhan search "Rust"

# 搜索并只看配置类问题
zhan search "nginx" --type config

# 组合搜索
zhan search "async" --type question --limit 20
```

## 场景二：提问并获取帮助

### 发布调试请求

```bash
# 使用模板创建调试帖子
zhan template show debug > debug_template.md

# 编辑模板内容
vim debug_template.md

# 发布
zhan post \
  --title "程序在处理 JSON 时崩溃" \
  --content "$(cat debug_template.md)" \
  --type debug \
  --tags "rust,json,panic" \
  --bounty 100
```

### 发布配置问题

```bash
zhan post \
  --title "Docker 容器无法连接到数据库" \
  --content "## 环境
- Ubuntu 22.04
- Docker 24.0
- PostgreSQL 15

## 问题
容器内的应用无法连接到宿主机上的数据库

## 配置
\`\`\`yaml
version: '3.8'
services:
  app:
    build: .
    network_mode: host
\`\`\`

## 错误
connection refused" \
  --type config \
  --tags "docker,postgresql,network"
```

### 发布代码审查请求

```bash
zhan post \
  --title "请审查我的 API 错误处理" \
  --content "## 概述
想请大家看看我的错误处理是否合理

## 代码
\`\`\`rust
async fn fetch_data() -> Result<Data, Error> {
    let response = client.get(url).await?;
    let data = response.json().await?;
    Ok(data)
}
\`\`\`

## 疑问
1. 错误类型是否合适?
2. 需要添加重试逻辑吗?" \
  --type code-review \
  --tags "rust,api,error-handling"
```

## 场景三：解决问题并获得悬赏

### 作为提问者：确认解决

当有人给出了有效的解决方案:

```bash
# 查看帖子详情，确认解决方案有效
zhan view 12345

# 确认解决并发放悬赏
zhan solved \
  --post-id 12345 \
  --bounty 100 \
  --time-saved 60
```

这会:
- 将帖子标记为"已解决"
- 支付 100 分 (1 元) 给回答者
- 记录节省时间 60 分钟

### 作为回答者：获得悬赏

```bash
# 1. 搜索可以帮助的问题
zhan search "rust" --type debug --limit 20

# 2. 查看问题详情
zhan view 12345

# 3. 回答问题 (在网页端或 API)

# 4. 如果被确认解决，会自动获得悬赏
```

## 场景四：查看个人数据

### 查看详细统计

```bash
zhan stats
```

输出示例:
```
📊 个人统计

发帖数: 45
已解决: 23
平均 CV: 12
声望值: 1280
账户余额: ¥56.50
```

### 定期检查收益

```bash
# 查看账户余额
zhan stats | grep -i "余额"
```

## 场景五：使用模板

### 快速生成帖子模板

```bash
# 生成 debug 模板
zhan template show debug

# 复制模板
zhan template show debug | pbcopy  # macOS
zhan template show debug | xclip   # Linux

# 查看所有可用模板
zhan template list
```

### 自定义模板

你也可以创建自己的模板文件:

```bash
# 创建自定义模板
mkdir -p ~/.config/zhan/templates
cat > ~/.config/zhan/templates/mydebug.md << 'EOF'
## 环境信息

OS: 
语言版本: 
框架版本: 

## 问题表现

<!-- 描述你遇到的问题 -->

## 复现步骤

1. 
2. 
3. 

## 错误日志

```
<!-- 粘贴错误信息 -->
```

## 已尝试的解决方案

- 
EOF

# 使用自定义模板
cat ~/.config/zhan/templates/mydebug.md | xargs -I {} zhan post --title "..." --content "{}" --type debug
```

## 场景六：自动化脚本

### 每日自动检查新帖子

```bash
#!/bin/bash
# 检查新的 debug 帖子

while true; do
  echo "=== 检查最新帖子 ==="
  zhan feed --type debug --limit 5
  echo ""
  sleep 300  # 每 5 分钟检查一次
done
```

### 关键词监控

```bash
#!/bin/bash
# 监控特定关键词

KEYWORDS=("rust" "performance" "async")
POST_IDS_FILE="/tmp/zhan_monitor_ids"

touch "$POST_IDS_FILE"

for keyword in "${KEYWORDS[@]}"; do
  echo "=== 搜索: $keyword ==="
  results=$(zhan search "$keyword" --limit 10 --type question)
  
  while IFS= read -r line; do
    if [[ "$line" =~ ID:\ ([0-9]+) ]]; then
      id="${BASH_REMATCH[1]}"
      if ! grep -q "$id" "$POST_IDS_FILE"; then
        echo "新帖子: $id - $keyword"
        echo "$id" >> "$POST_IDS_FILE"
      fi
    fi
  done <<< "$results"
done
```

## 场景七：快捷别名

在 shell 配置文件中添加别名:

```bash
# ~/.bashrc 或 ~/.zshrc

# 快速命令
alias zf='zhan feed'
alias zs='zhan search'
alias zv='zhan view'
alias zw='zhan whoami'

# 常用搜索
alias zfrust='zhan search "Rust"'
alias zfdebug='zhan feed --type debug'
alias zfquestion='zhan feed --type question'
```

然后重新加载配置:

```bash
source ~/.bashrc  # 或 ~/.zshrc
```

现在可以快速使用:

```bash
zf              # 查看 feed
zfrust          # 搜索 Rust
zfdebug --limit 5  # 查看 5 条调试帖子
```