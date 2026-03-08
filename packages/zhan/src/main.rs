use anyhow::Result;
use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;
use zhan_sdk::{ApiClient, Config, CoffeeInput, CreatePostInput, DeviceLogin, PostType, RewardInput, SolvedInput, UserStats};

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
    /// 发布新帖子
    Post {
        /// 帖子标题
        title: String,
        /// 帖子内容 (Markdown)
        content: String,
        /// 帖子类型: debug, code-review, config, question
        #[arg(long, default_value = "question")]
        r#type: String,
        /// 标签 (逗号分隔)
        #[arg(long)]
        tags: Option<String>,
        /// 悬赏金额 (分)
        #[arg(long)]
        bounty: Option<i64>,
    },
    /// 确认帖子帮你解决了问题
    Solved {
        /// 帖子 ID
        post_id: String,
        /// 悬赏金额 (分)
        #[arg(long)]
        bounty: Option<i64>,
        /// 节省的时间 (分钟)
        #[arg(long)]
        time_saved: Option<i64>,
    },
    /// 将悬赏发放给指定用户
    Reward {
        /// 帖子 ID
        post_id: String,
        /// 回答者用户 ID
        #[arg(long)]
        to: String,
    },
    /// 显示配置文件路径
    /// 显示个人统计
    Stats,
    /// 请作者喝咖啡
    Coffee {
        /// 帖子 ID
        post_id: String,
        /// 金额 (分)
        amount: Option<i64>,
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
        Commands::Post { title, content, r#type, tags, bounty } => {
            println!("{}", "发布帖子...".bold());
            
            // 检查登录
            let config = match Config::load() {
                Ok(c) => c,
                Err(_) => {
                    println!("{}", "✗ 请先登录".red());
                    return Ok(());
                }
            };
            
            if !config.is_logged_in() {
                println!("{}", "✗ 请先运行 `zhan login` 登录".red());
                return Ok(());
            }
            
            // 解析类型
            let post_type = match r#type.as_str() {
                "debug" => PostType::Debug,
                "code-review" | "codereview" | "code_review" => PostType::CodeReview,
                "config" => PostType::Config,
                "question" | "q" => PostType::Question,
                _ => {
                    println!("{} {}", "✗ 无效类型:".red(), r#type);
                    println!("有效类型: debug, code-review, config, question");
                    return Ok(());
                }
            };
            
            // 解析标签
            let tags_vec = tags.map(|t| {
                t.split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            });
            
            let input = CreatePostInput {
                title,
                content_md: content,
                post_type,
                tags: tags_vec,
                bounty_cents: bounty,
            };
            
            let client = match ApiClient::new() {
                Ok(c) => c,
                Err(e) => {
                    println!("{} {}", "✗ 创建客户端失败:".red(), e);
                    return Ok(());
                }
            };
            
            match client.create_post(&input).await {
                Ok(result) => {
                    println!("{}", "✓ 帖子发布成功！".green());
                    println!("  ID: {}", result.id);
                    println!("  URL: {}", result.url);
                    if let Some(warnings) = &result.template_warnings {
                        if !warnings.is_empty() {
                            println!("\n⚠️  模板建议:");
                            for w in warnings {
                                println!("  - {}", w);
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("{} {}", "✗ 发布失败:".red(), e);
                }
            }
        }
        Commands::Solved { post_id, bounty, time_saved } => {
            println!("{}", "确认帖子解决...".bold());
            
            let config = match Config::load() {
                Ok(c) => c,
                Err(_) => {
                    println!("{}", "✗ 请先登录".red());
                    return Ok(());
                }
            };
            
            if !config.is_logged_in() {
                println!("{}", "✗ 请先运行 `zhan login` 登录".red());
                return Ok(());
            }
            
            let input = SolvedInput {
                bounty_cents: bounty,
                time_saved_minutes: time_saved,
            };
            
            let client = match ApiClient::new() {
                Ok(c) => c,
                Err(e) => {
                    println!("{} {}", "✗ 创建客户端失败:".red(), e);
                    return Ok(());
                }
            };
            
            match client.solved(&post_id, &input).await {
                Ok(result) => {
                    println!("{}", "✓ 确认成功！".green());
                    println!("  Solved ID: {}", result.solved_id);
                    println!("  帖子 {} 的 Solved 数: {}", result.post_id, result.new_solved_count);
                }
                Err(e) => {
                    println!("{} {}", "✗ 确认失败:".red(), e);
                }
            }
        }
        Commands::Reward { post_id, to } => {
            println!("{}", "发放悬赏...".bold());
            
            let config = match Config::load() {
                Ok(c) => c,
                Err(_) => {
                    println!("{}", "✗ 请先登录".red());
                    return Ok(());
                }
            };
            
            if !config.is_logged_in() {
                println!("{}", "✗ 请先运行 `zhan login` 登录".red());
                return Ok(());
            }
            
            let input = RewardInput {
                answerer_user_id: to,
            };
            
            let client = match ApiClient::new() {
                Ok(c) => c,
                Err(e) => {
                    println!("{} {}", "✗ 创建客户端失败:".red(), e);
                    return Ok(());
                }
            };
            
            match client.reward(&post_id, &input).await {
                Ok(result) => {
                    println!("{}", "✓ 悬赏发放成功！".green());
                    println!("  {:?}", result);
                }
                Err(e) => {
                    println!("{} {}", "✗ 发放失败:".red(), e);
                }
            }
        }
        Commands::Stats => {
            println!("{}", "获取统计数据...".bold());
            
            let config = match Config::load() {
                Ok(c) => c,
                Err(_) => {
                    println!("{}", "✗ 请先登录".red());
                    return Ok(());
                }
            };
            
            if !config.is_logged_in() {
                println!("{}", "✗ 请先运行 `zhan login` 登录".red());
                return Ok(());
            }
            
            let client = match ApiClient::new() {
                Ok(c) => c,
                Err(e) => {
                    println!("{} {}", "✗ 创建客户端失败:".red(), e);
                    return Ok(());
                }
            };
            
            match client.get_stats().await {
                Ok(stats) => {
                    let level = if stats.avg_cvs >= 0.7 { "High" } else if stats.avg_cvs >= 0.5 { "Medium" } else { "Low" };
                    let balance_usd = stats.balance_cents as f64 / 100.0;
                    let coffee_usd = stats.total_coffee_cents as f64 / 100.0;
                    
                    println!("\n@{} 的数据", config.username.as_ref().unwrap_or(&"unknown".to_string()));
                    println!("CVS: {:.2} ({}) | 收益: ${:.2} | 帖子: {} | Solved: {}",
                        stats.avg_cvs, level, coffee_usd, stats.post_count, stats.total_solved_count);
                    println!("7日发帖: {} | 声誉: {:.2} | 余额: ${:.2}",
                        stats.recent_post_count, stats.reputation, balance_usd);
                }
                Err(e) => {
                    println!("{} {}", "✗ 获取统计失败:".red(), e);
                }
            }
        }
        Commands::Coffee { post_id, amount } => {
            println!("{}", "请作者喝咖啡...".bold());
            
            let config = match Config::load() {
                Ok(c) => c,
                Err(_) => {
                    println!("{}", "✗ 请先登录".red());
                    return Ok(());
                }
            };
            
            if !config.is_logged_in() {
                println!("{}", "✗ 请先运行 `zhan login` 登录".red());
                return Ok(());
            }
            
            let client = match ApiClient::new() {
                Ok(c) => c,
                Err(e) => {
                    println!("{} {}", "✗ 创建客户端失败:".red(), e);
                    return Ok(());
                }
            };
            
            match client.coffee(&post_id, amount).await {
                Ok(result) => {
                    println!("{}", "✓ 支付会话已创建！".green());
                    println!("  订单 ID: {}", result.coffee_id);
                    println!("  支付链接: {}", result.checkout_url);
                    println!("\n请在浏览器中打开链接完成支付 ☕");
                }
                Err(e) => {
                    println!("{} {}", "✗ 创建支付会话失败:".red(), e);
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