use anyhow::Result;
use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;
use zhan_sdk::{ApiClient, Config, DeviceLogin};

#[derive(Parser)]
#[command(name = "zhan")]
#[command(about = "栈间 Zhanjian CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 通过设备码登录
    Login {
        /// 直接使用 API Token 登录
        #[arg(long)]
        token: Option<String>,
    },
    /// 显示当前登录用户信息
    Whoami,
    /// 退出登录
    Logout,
    /// 检查 API 连接状态
    Health,
    /// 浏览社区 Feed
    Feed {
        /// 过滤类型
        #[arg(long)]
        r#type: Option<String>,
        /// 返回数量
        #[arg(long)]
        limit: Option<u32>,
    },
    /// 搜索帖子
    Search {
        /// 搜索关键词
        query: String,
    },
    /// 查看帖子详情
    View {
        /// 帖子 ID
        post_id: String,
    },
    /// 显示配置文件路径
    ConfigPath,
    /// 查看配置
    Config {
        /// 配置键名
        key: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Login { token } => {
            println!("{}", "设备码登录".bold());
            
            if let Some(token) = token {
                println!("使用 Token 登录...");
                match DeviceLogin::new() {
                    Ok(login) => {
                        match login.login_with_token(&token).await {
                            Ok(result) => {
                                println!();
                                println!("{} {}", "✓".green(), "登录成功！");
                                println!("  欢迎 @{}", result.username.bold());
                            }
                            Err(e) => {
                                println!("{} {}", "✗ 登录失败:".red(), e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("{} {}", "✗ 初始化失败:".red(), e);
                    }
                }
            } else {
                println!("启动设备码登录...\n");
                
                match DeviceLogin::new() {
                    Ok(login) => {
                        match login.start().await {
                            Ok(device_code) => {
                                println!("{}", "─".dimmed());
                                println!("  验证地址: {}", device_code.verification_uri_complete.as_ref().unwrap_or(&device_code.verification_uri).cyan());
                                println!("  设备码: {}", device_code.user_code.bold());
                                println!("  有效期: {} 分钟", device_code.expires_in / 60);
                                println!("{}", "─".dimmed());
                                println!();
                                println!("请在浏览器中打开验证地址，确认登录请求");
                                println!();
                                println!("等待确认...\n");
                                
                                match login.poll(&device_code.device_code).await {
                                    Ok(result) => {
                                        println!();
                                        println!("{} {}", "✓".green(), "登录成功！");
                                        println!("  欢迎 @{}", result.username.bold());
                                    }
                                    Err(e) => {
                                        println!("{} {}", "✗ 登录失败:".red(), e);
                                    }
                                }
                            }
                            Err(e) => {
                                println!("{} {}", "✗ 启动登录失败:".red(), e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("{} {}", "✗ 初始化失败:".red(), e);
                    }
                }
            }
        }
        Commands::Whoami => {
            println!("{}", "获取用户信息...".bold());
            
            let config = match Config::load() {
                Ok(c) => c,
                Err(_) => {
                    println!("{}", "✗ 无法加载配置".red());
                    return Ok(());
                }
            };
            
            if !config.is_logged_in() {
                println!("{}", "未登录，请运行 `zhan login`".yellow());
                return Ok(());
            }
            
            match ApiClient::new() {
                Ok(client) => {
                    match client.get_current_user().await {
                        Ok(user) => {
                            println!("\n{}", format!("  @{}", user.username).bold());
                            if let Some(email) = &user.email {
                                println!("  邮箱: {}", email);
                            }
                            if let Some(avatar) = &user.avatar_url {
                                println!("  头像: {}", avatar);
                            }
                        }
                        Err(e) => {
                            println!("{} {}", "✗".red(), e);
                        }
                    }
                }
                Err(e) => {
                    println!("{} {}", "✗ 创建客户端失败:".red(), e);
                }
            }
        }
        Commands::Logout => {
            println!("{}", "退出登录".bold());
            let mut config = match Config::load() {
                Ok(c) => c,
                Err(_) => {
                    println!("{}", "未登录，无需退出".yellow());
                    return Ok(());
                }
            };
            
            config.clear_token();
            if let Err(e) = config.save() {
                println!("{} {}", "✗ 保存配置失败:".red(), e);
                return Ok(());
            }
            
            println!("{}", "✓ 已退出登录".green());
        }
        Commands::Health => {
            println!("{}", "检查 API 状态...".bold());
            
            let client = match ApiClient::new() {
                Ok(c) => c,
                Err(e) => {
                    println!("{} {}", "✗ 创建客户端失败:".red(), e);
                    return Ok(());
                }
            };
            
            match client.health().await {
                Ok(response) => {
                    println!("\n{}", "  API 状态".bold());
                    println!("  ─────────");
                    println!("  status: {}", response.status.green());
                    if let Some(version) = response.version {
                        println!("  version: {}", version);
                    }
                    println!();
                }
                Err(e) => {
                    println!("{} {}", "✗".red(), e);
                    println!("\n提示: 请确认 API 服务正在运行");
                }
            }
        }
        Commands::Feed { r#type, limit } => {
            println!("{}", "浏览 Feed".bold());
            
            let client = match ApiClient::new() {
                Ok(c) => c,
                Err(e) => {
                    println!("{} {}", "✗ 创建客户端失败:".red(), e);
                    return Ok(());
                }
            };
            
            let type_str = r#type.as_deref();
            
            match client.get_feed(type_str, limit).await {
                Ok(posts) => {
                    if posts.is_empty() {
                        println!("暂无内容");
                    } else {
                        println!();
                        for (i, post) in posts.iter().enumerate() {
                            let author_name = post.author.as_ref()
                                .map(|a| a.username.as_str())
                                .unwrap_or("unknown");
                            
                            println!("{}. {}", (i + 1).to_string().bold(), post.title);
                            println!("   @{} · {} · 👁 {} · ❤️ {} · 💬 {}", 
                                author_name,
                                &post.created_at[..10],
                                post.view_count,
                                post.like_count.unwrap_or(0),
                                post.comment_count.unwrap_or(0)
                            );
                            if !post.tags.is_empty() {
                                println!("   标签: {}", post.tags.join(", "));
                            }
                            println!();
                        }
                    }
                }
                Err(e) => {
                    println!("{} {}", "✗".red(), e);
                }
            }
        }
        Commands::Search { query } => {
            println!("{}", format!("搜索: {}", query).bold());
            
            let client = match ApiClient::new() {
                Ok(c) => c,
                Err(e) => {
                    println!("{} {}", "✗ 创建客户端失败:".red(), e);
                    return Ok(());
                }
            };
            
            match client.search(&query, None, None, None).await {
                Ok(result) => {
                    if result.posts.is_empty() {
                        println!("未找到相关帖子");
                    } else {
                        println!("找到 {} 个结果\n", result.total);
                        for (i, post) in result.posts.iter().enumerate() {
                            let author_name = post.author.as_ref()
                                .map(|a| a.username.as_str())
                                .unwrap_or("unknown");
                            
                            println!("{}. {}", (i + 1).to_string().bold(), post.title);
                            println!("   @{} · {} · 👁 {}", 
                                author_name,
                                &post.created_at[..10],
                                post.view_count
                            );
                            println!();
                        }
                    }
                }
                Err(e) => {
                    println!("{} {}", "✗".red(), e);
                }
            }
        }
        Commands::View { post_id } => {
            println!("{}", format!("查看帖子: {}", post_id).bold());
            
            let client = match ApiClient::new() {
                Ok(c) => c,
                Err(e) => {
                    println!("{} {}", "✗ 创建客户端失败:".red(), e);
                    return Ok(());
                }
            };
            
            match client.get_post(&post_id).await {
                Ok(post) => {
                    let author_name = post.author.as_ref()
                        .map(|a| a.username.as_str())
                        .unwrap_or("unknown");
                    
                    println!();
                    println!("{}", post.title.bold());
                    println!("@{} · {}", author_name, &post.created_at[..10]);
                    println!();
                    
                    // 显示内容（安全截断，避免 UTF-8 截断）
                    let content = post.content.as_ref().or(post.content_md.as_ref());
                    if let Some(c) = content {
                        let chars: Vec<char> = c.chars().collect();
                        let display_content = if chars.len() > 500 {
                            chars[..500].iter().collect::<String>() + "..."
                        } else {
                            c.clone()
                        };
                        println!("{}", display_content);
                        println!();
                    }
                    
                    // 显示标签
                    if !post.tags.is_empty() {
                        println!("标签: {}", post.tags.join(", "));
                    }
                    
                    // 显示统计
                    println!("\n👁 {} · ❤️ {} · 💬 {}", 
                        post.view_count, 
                        post.like_count.unwrap_or(0), 
                        post.comment_count.unwrap_or(0)
                    );
                }
                Err(e) => {
                    println!("{} {}", "✗".red(), e);
                }
            }
        }
        Commands::ConfigPath => {
            let path = Config::path();
            println!("配置文件路径:");
            println!("  {}", path.display());
        }
        Commands::Config { key } => {
            let config = match Config::load() {
                Ok(c) => c,
                Err(e) => {
                    println!("{} {}", "✗ 加载配置失败:".red(), e);
                    return Ok(());
                }
            };
            
            if let Some(k) = key {
                match k.as_str() {
                    "token" => {
                        if let Some(t) = config.token {
                            println!("token: {}", t.chars().take(10).collect::<String>() + "...");
                        } else {
                            println!("token: (未设置)");
                        }
                    }
                    "apiUrl" | "api_url" | "api-url" => {
                        println!("apiUrl: {}", config.api_url);
                    }
                    "username" => {
                        if let Some(u) = config.username {
                            println!("username: @{}", u);
                        } else {
                            println!("username: (未设置)");
                        }
                    }
                    _ => {
                        println!("未知配置项: {}", k);
                    }
                }
            } else {
                println!("配置项:");
                if let Some(t) = config.token {
                    println!("  token: {}", t.chars().take(10).collect::<String>() + "...");
                } else {
                    println!("  token: (未设置)");
                }
                println!("  apiUrl: {}", config.api_url);
                if let Some(u) = config.username {
                    println!("  username: @{}", u);
                }
            }
        }
    }

    Ok(())
}