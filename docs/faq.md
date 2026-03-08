# 常见问题解答 (FAQ)

本文档解答关于 zhan-cli 的常见问题。

## 安装问题

### Q: 安装脚本执行失败

**A:** 尝试以下解决方案:

```bash
# 1. 检查 curl 是否安装
which curl

# 2. 如果权限问题，尝试手动安装
curl -sSL -o zhan.tar.gz https://github.com/1BossX/zhan-cli-rust/releases/latest/download/zhan-x86_64-unknown-linux-gnu.tar.gz
tar -xzf zhan.tar.gz
chmod +x zhan
sudo mv zhan /usr/local/bin/

# 3. 或者安装到用户目录
mkdir -p ~/.local/bin
mv zhan ~/.local/bin/
export PATH="$HOME/.local/bin:$PATH"  # 添加到 ~/.bashrc
```

---

### Q: "command not found" 错误

**A:** 确保二进制文件在 PATH 中:

```bash
# 检查 zhan 是否在 PATH
which zhan

# 如果不在，添加路径
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

---

### Q: 安装后运行显示 "No such file or directory"

**A:** 可能是缺少动态库。尝试从源码编译安装:

```bash
cargo install --package zhan --locked
```

---

## 登录问题

### Q: 设备码登录超时

**A:** 设备码有效期为 5 分钟。请在超时前完成浏览器授权。如果超时，重新运行 `zhan login`。

---

### Q: 如何获取 API Token?

**A:** 
1. 登录网页端栈间
2. 进入个人设置
3. 找到 "API Token" 或 "开发者设置"
4. 创建新 Token

---

### Q: Token 过期怎么办?

**A:** 
1. 重新登录: `zhan logout && zhan login`
2. 或者使用新 Token: `zhan login --token <NEW_TOKEN>`

---

### Q: 登录后显示 "not logged in"

**A:** 检查配置文件是否正确:

```bash
cat ~/.config/zhan/config.toml
```

如果文件不存在或内容错误，手动创建:

```toml
[api]
base_url = "https://api.zhanjian.com"
token = "your-token-here"
```

---

## 使用问题

### Q: 帖子内容如何格式化?

**A:** 支持 Markdown 格式:

```markdown
## 标题

正文内容

- 列表项 1
- 列表项 2

\`\`\`rust
fn main() {
    println!("Hello");
}
\`\`\`
```

---

### Q: 如何设置帖子标签?

**A:** 使用 `--tags` 选项，逗号分隔多个标签:

```bash
zhan post \
  --title "问题标题" \
  --content "内容..." \
  --tags "rust,async,performance"
```

---

### Q: 悬赏金额单位是什么?

**A:** 悬赏金额以"分"为单位 (1 元 = 100 分):

```bash
# 悬赏 5 元
--bounty 500

# 悬赏 1 元
--bounty 100
```

---

### Q: 如何删除已发布的帖子?

**A:** 目前 CLI 不支持删除帖子，请在网页端操作。

---

### Q: 搜索结果为空

**A:** 尝试以下方法:

1. 减少关键词
2. 不指定类型 (默认搜索全部类型)
3. 检查关键词拼写

---

## 技术问题

### Q: 如何查看 API 请求详情?

**A:** 使用 `--verbose` 选项 (如果支持):

```bash
zhan feed --verbose
```

或者查看日志:

```bash
# Linux
tail -f ~/.local/share/zhan/logs/*.log

# 或直接查看配置目录
ls -la ~/.config/zhan/
```

---

### Q: 配置文件在哪里?

**A:** 
- Linux: `~/.config/zhan/config.toml`
- macOS: `~/.config/zhan/config.toml`
- Windows: `%APPDATA%\zhan\config.toml`

---

### Q: 如何更新 zhan-cli?

```bash
# 方式一: 重新运行安装脚本
curl -sSL https://raw.githubusercontent.com/1BossX/zhan-cli-rust/main/install.sh | sh

# 方式二: 从源码更新
cd zhan-cli-rust
git pull origin main
cargo install --package zhan --locked

# 方式三: 手动下载
# 访问 https://github.com/1BossX/zhan-cli-rust/releases 下载最新版本
```

---

### Q: 如何查看版本号?

```bash
zhan --version
```

---

## 贡献与反馈

### Q: 如何报告 Bug?

请在 GitHub Issues 中报告，包含以下信息:

1. 操作系统和版本
2. zhan-cli 版本 (`zhan --version`)
3. 复现步骤
4. 错误信息

---

### Q: 如何贡献代码?

1. Fork 项目
2. 创建功能分支: `git checkout -b feature/xxx`
3. 提交更改: `git commit -m 'Add xxx'`
4. 推送分支: `git push origin feature/xxx`
5. 创建 Pull Request

---

### Q: 如何联系开发者?

- Discord: 加入栈间社区
- GitHub: 提 Issue
- 邮件: 联系主页显示的邮箱

---

## 其他问题

### Q: 为什么命令输出是英文的?

A: 目前 CLI 输出主要是英文，但会逐步增加中文支持。

---

### Q: 有移动端 App 吗?

A: 目前只有 CLI 和网页端，移动端 App 正在开发中。

---

### Q: 如何支持这个项目?

- ⭐ Star 项目
- ☕ 请作者喝咖啡: `zhan coffee`
- 📢 推荐给朋友
- 💻 贡献代码