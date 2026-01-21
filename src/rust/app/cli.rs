use crate::config::load_standalone_telegram_config;
use crate::telegram::handle_telegram_only_mcp_request;
use crate::log_important;
use crate::app::builder::run_tauri_app;
use anyhow::Result;

/// 处理命令行参数
pub fn handle_cli_args() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        // 无参数：正常启动GUI
        1 => {
            run_tauri_app();
        }
        // 单参数：帮助或版本
        2 => {
            match args[1].as_str() {
                "--help" | "-h" => print_help(),
                "--version" | "-v" => print_version(),
                _ => {
                    eprintln!("未知参数: {}", args[1]);
                    print_help();
                    std::process::exit(1);
                }
            }
        }
        // 多参数：MCP请求模式或图标搜索模式
        _ => {
            if args[1] == "--mcp-request" && args.len() >= 3 {
                handle_mcp_request(&args[2])?;
            } else if args[1] == "--icon-search" {
                // 图标搜索模式：解析参数并启动 GUI
                handle_icon_search(&args[2..])?;
            } else {
                eprintln!("无效的命令行参数");
                print_help();
                std::process::exit(1);
            }
        }
    }

    Ok(())
}

/// 处理MCP请求
fn handle_mcp_request(request_file: &str) -> Result<()> {
    // 检查Telegram配置，决定是否启用纯Telegram模式
    match load_standalone_telegram_config() {
        Ok(telegram_config) => {
            if telegram_config.enabled && telegram_config.hide_frontend_popup {
                // 纯Telegram模式：不启动GUI，直接处理
                if let Err(e) = tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(handle_telegram_only_mcp_request(request_file))
                {
                    log_important!(error, "处理Telegram请求失败: {}", e);
                    std::process::exit(1);
                }
            } else {
                // 正常模式：启动GUI处理弹窗
                run_tauri_app();
            }
        }
        Err(e) => {
            log_important!(warn, "加载Telegram配置失败: {}，使用默认GUI模式", e);
            // 配置加载失败时，使用默认行为（启动GUI）
            run_tauri_app();
        }
    }
    Ok(())
}

/// 处理图标搜索请求
/// 
/// 解析 CLI 参数并设置环境变量，启动 GUI 进入图标选择模式
fn handle_icon_search(args: &[String]) -> Result<()> {
    // 解析参数
    let mut query = String::new();
    let mut style = String::new();
    let mut save_path = String::new();
    let mut project_root = String::new();
    
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--query" if i + 1 < args.len() => {
                query = args[i + 1].clone();
                i += 2;
            }
            "--style" if i + 1 < args.len() => {
                style = args[i + 1].clone();
                i += 2;
            }
            "--save-path" if i + 1 < args.len() => {
                save_path = args[i + 1].clone();
                i += 2;
            }
            "--project-root" if i + 1 < args.len() => {
                project_root = args[i + 1].clone();
                i += 2;
            }
            _ => {
                // 如果第一个参数不是选项，假设它是搜索关键词
                if i == 0 && !args[i].starts_with("--") {
                    query = args[i].clone();
                }
                i += 1;
            }
        }
    }
    
    // 设置环境变量，供 Tauri 应用读取
    std::env::set_var("SANSHU_ICON_MODE", "true");
    if !query.is_empty() {
        std::env::set_var("SANSHU_ICON_QUERY", &query);
    }
    if !style.is_empty() {
        std::env::set_var("SANSHU_ICON_STYLE", &style);
    }
    if !save_path.is_empty() {
        std::env::set_var("SANSHU_ICON_SAVE_PATH", &save_path);
    }
    if !project_root.is_empty() {
        std::env::set_var("SANSHU_ICON_PROJECT_ROOT", &project_root);
    }
    
    // 启动 GUI 进入图标选择模式
    run_tauri_app();
    
    Ok(())
}

/// 显示帮助信息
fn print_help() {
    println!("三术 - 智能代码审查工具");
    println!();
    println!("用法:");
    println!("  等一下                              启动设置界面");
    println!("  等一下 --mcp-request <文件>          处理 MCP 请求");
    println!("  等一下 --icon-search [选项]          打开图标选择界面");
    println!("  等一下 --help                       显示此帮助信息");
    println!("  等一下 --version                    显示版本信息");
    println!();
    println!("图标搜索选项:");
    println!("  --query <关键词>      预设搜索关键词");
    println!("  --style <风格>        图标风格: line/fill/flat/all");
    println!("  --save-path <路径>    保存目录路径");
    println!("  --project-root <路径> 项目根目录");
}

/// 显示版本信息
fn print_version() {
    println!("三术 v{}", env!("CARGO_PKG_VERSION"));
}

