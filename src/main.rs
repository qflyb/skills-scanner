mod skill;
mod scanner;
mod ui;

use std::path::PathBuf;
use clap::Parser;
use console::style;

use scanner::Scanner;

/// 扫描并管理本地 AI 工具的 skills
#[derive(Parser)]
#[command(name = "skills-scanner")]
#[command(version = "0.1.0")]
#[command(about = "扫描并管理本地 AI 工具的 skills", long_about = None)]
struct Cli {
    /// 自定义扫描目录，可多次指定
    #[arg(short, long, value_name = "DIR")]
    path: Option<Vec<PathBuf>>,

    /// 仅列出 skills，不进入交互模式
    #[arg(short, long)]
    list: bool,
}

fn main() {
    let cli = Cli::parse();

    // 创建扫描器
    let mut scanner = Scanner::new();

    // 添加自定义路径
    if let Some(paths) = cli.path {
        for path in paths {
            scanner.add_custom_path(path);
        }
    }

    // 显示扫描信息
    ui::show_scanning_message(scanner.path_count());

    // 执行扫描
    let mut skills = scanner.scan();

    if cli.list {
        // 仅列出模式
        ui::display_skills(&skills);
        return;
    }

    // 交互模式循环
    loop {
        match ui::show_main_menu(skills.len()) {
            Ok(ui::MainMenuAction::BrowseAll) => {
                let all_indices: Vec<usize> = (0..skills.len()).collect();
                match ui::interactive_select_and_delete(&skills, &all_indices) {
                    Ok(true) => {
                        println!(
                            "\n{} 正在重新扫描...\n",
                            style("🔍").cyan()
                        );
                        skills = scanner.scan();
                    }
                    Ok(false) => {}
                    Err(e) => eprintln!("操作出错: {}", e),
                }
            }
            Ok(ui::MainMenuAction::Exit) => {
                println!("\n{}\n", style("再见!").green());
                break;
            }
            Err(e) => {
                eprintln!("菜单出错: {}", e);
                break;
            }
        }
    }
}
