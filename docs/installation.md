# 安装指南

本文档详细介绍 zhan-cli 的各种安装方式。

## 系统要求

- **操作系统**: Linux / macOS / Windows (WSL)
- **架构**: x86_64 或 ARM64
- **依赖**: 无需额外依赖（静态编译）

## 安装方式

### 方式一: 一键安装脚本 (推荐)

```bash
# macOS / Linux
curl -sSL https://raw.githubusercontent.com/1BossX/zhan-cli-rust/main/install.sh | sh
```

安装脚本会:
1. 自动检测系统平台和架构
2. 下载对应版本的二进制文件
3. 安装到 `~/.local/bin` 或 `/usr/local/bin`
4. 添加到 PATH

### 方式二: 从 Release 下载

访问 [Releases 页面](https://github.com/1BossX/zhan-cli-rust/releases) 下载预编译的二进制文件：

| 平台 | 文件名 |
|------|--------|
| Linux x86_64 | `zhan-x86_64-unknown-linux-gnu.tar.gz` |
| Linux ARM64 | `zhan-aarch64-unknown-linux-gnu.tar.gz` |
| macOS Intel | `zhan-x86_64-apple-darwin.tar.gz` |
| macOS Apple Silicon | `zhan-aarch64-apple-darwin.tar.gz` |
| Windows | `zhan-x86_64-pc-windows-gnu.zip` |

**手动安装步骤:**

```bash
# 1. 解压
tar -xzf zhan-x86_64-unknown-linux-gnu.tar.gz

# 2. 移动到 PATH 目录
chmod +x zhan
sudo mv zhan /usr/local/bin/

# 3. 验证安装
zhan --version
```

### 方式三: 从源码编译

需要先安装 Rust 工具链:

```bash
# 安装 Rust (如果没有)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 克隆项目
git clone https://github.com/1BossX/zhan-cli-rust.git
cd zhan-cli-rust

# 编译安装
cargo install --package zhan --locked
```

### 方式四: Homebrew (macOS)

```bash
# 即将支持
# brew install zhan-cli
```

## 安装验证

安装完成后，运行以下命令验证:

```bash
zhan --version
zhan --help
```

## 卸载

```bash
# 删除二进制文件
sudo rm /usr/local/bin/zhan

# 或使用一键卸载脚本 (如果使用方式一安装)
curl -sSL https://raw.githubusercontent.com/1BossX/zhan-cli-rust/main/uninstall.sh | sh
```

## 配置文件

zhan-cli 会将配置保存在以下位置:

- **Linux/macOS**: `~/.config/zhan/config.toml`
- **Windows**: `%APPDATA%\zhan\config.toml`

配置文件示例:

```toml
[api]
base_url = "https://api.zhanjian.com"
token = "your-api-token-here"

[user]
username = "your-username"
```

## 故障排查

### 命令未找到

确保 `~/.local/bin` 或 `/usr/local/bin` 在你的 PATH 中:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

### 权限被拒绝

如果安装到 `/usr/local/bin` 时遇到权限问题:

```bash
sudo chown -R $(whoami) /usr/local/bin
```

### 更新版本

```bash
# 重新运行安装脚本即可更新
curl -sSL https://raw.githubusercontent.com/1BossX/zhan-cli-rust/main/install.sh | sh

# 或手动下载新版本替换
```