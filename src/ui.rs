use std::fs;
use std::collections::{BTreeMap, HashSet};
use console::{style, Key, Term};
use dialoguer::{MultiSelect, Confirm, Select, theme::ColorfulTheme};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use crate::skill::Skill;

pub enum MainMenuAction {
    BrowseAll,
    Exit,
}

struct SkillGroup {
    name: String,
    indices: Vec<usize>,
}

/// 确保光标恢复显示的 RAII guard
struct CursorGuard<'a>(&'a Term);

impl<'a> Drop for CursorGuard<'a> {
    fn drop(&mut self) {
        let _ = self.0.show_cursor();
    }
}

/// 显示主菜单
pub fn show_main_menu(skill_count: usize) -> Result<MainMenuAction> {
    println!(
        "\n{} 共 {} 个 skills\n",
        style("📦").cyan(),
        style(skill_count).green().bold()
    );

    let items = vec!["浏览所有 skills", "退出"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("请选择操作")
        .items(&items)
        .default(0)
        .interact()?;

    match selection {
        0 => Ok(MainMenuAction::BrowseAll),
        _ => Ok(MainMenuAction::Exit),
    }
}

/// 按名称分组 skills
fn group_skills(skills: &[Skill], indices: &[usize]) -> Vec<SkillGroup> {
    let mut map: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for &idx in indices {
        let name = skills[idx].name.clone();
        map.entry(name).or_default().push(idx);
    }
    map.into_iter()
        .map(|(name, indices)| SkillGroup { name, indices })
        .collect()
}

/// 交互式选择并删除 skills（完整流程）
/// 返回 Ok(true) 表示有 skills 被删除，需要重新扫描
pub fn interactive_select_and_delete(skills: &[Skill], indices: &[usize]) -> Result<bool> {
    if indices.is_empty() {
        println!("{}", style("未找到任何 skills").yellow());
        return Ok(false);
    }

    let groups = group_skills(skills, indices);

    let term = Term::stdout();

    let total_skills: usize = groups.iter().map(|g| g.indices.len()).sum();

    // 第一级：选择分组
    let group_items: Vec<String> = groups
        .iter()
        .map(|g| {
            let tools: Vec<&str> = g.indices.iter().map(|&i| skills[i].tool.as_str()).collect();
            if tools.len() == 1 {
                let skill = &skills[g.indices[0]];
                let desc = skill.display_description();
                let truncated_desc = if desc.len() > 40 {
                    format!("{}...", &desc[..37])
                } else {
                    desc.to_string()
                };
                format!(
                    "{:<12} > {:<20} {}",
                    tools[0],
                    g.name,
                    style(truncated_desc).dim()
                )
            } else {
                format!(
                    "{:<35} {}",
                    g.name,
                    style(format!("{}个来源: {}", tools.len(), tools.join(", "))).dim()
                )
            }
        })
        .collect();

    let search_keys: Vec<String> = groups.iter().map(|g| g.name.clone()).collect();

    // 构建静态头部信息
    let header_lines = vec![
        String::new(),
        format!(
            "{} 找到 {} 个 skills ({} 个分组):",
            style("🔍").cyan(),
            style(total_skills).green().bold(),
            style(groups.len()).green()
        ),
        String::new(),
        format!("{}", style("━".repeat(60)).dim()),
        format!(
            "  {} 选择/取消  {} 移动  {} 确认  {} 实时搜索",
            style("空格").cyan().bold(),
            style("↑↓").cyan().bold(),
            style("Enter").cyan().bold(),
            style("键入字符").cyan().bold(),
        ),
        format!(
            "  {} 单来源项直接删除，多来源项可展开选择子项",
            style("提示:").yellow()
        ),
        format!("{}", style("━".repeat(60)).dim()),
        String::new(),
    ];

    let selected_group_indices = searchable_multi_select(
        &term,
        &group_items,
        &search_keys,
        "选择要删除的 skills",
        &header_lines,
    )?;

    if selected_group_indices.is_empty() {
        println!("\n未选择任何 skill\n");
        return Ok(false);
    }

    // 第二级：对多来源分组，询问具体要删除哪些
    let mut final_skill_indices: Vec<usize> = Vec::new();

    for &gi in &selected_group_indices {
        let group = &groups[gi];
        if group.indices.len() == 1 {
            final_skill_indices.push(group.indices[0]);
        } else {
            println!(
                "\n{} {} 有 {} 个来源，选择要删除的:\n",
                style("▶").cyan(),
                style(&group.name).white().bold(),
                style(group.indices.len()).green()
            );

            let mut sub_items: Vec<String> =
                vec![format!("{}", style("★ 全部删除").red().bold())];
            for &idx in &group.indices {
                let skill = &skills[idx];
                let desc = skill.display_description();
                let truncated_desc = if desc.len() > 40 {
                    format!("{}...", &desc[..37])
                } else {
                    desc.to_string()
                };
                sub_items.push(format!(
                    "{:<12} > {:<20} {}",
                    skill.tool,
                    skill.name,
                    style(truncated_desc).dim()
                ));
            }

            let sub_selections = MultiSelect::with_theme(&ColorfulTheme::default())
                .with_prompt(format!("{} - 选择来源", group.name))
                .items(&sub_items)
                .interact()?;

            if sub_selections.contains(&0) {
                final_skill_indices.extend(&group.indices);
            } else {
                for &si in &sub_selections {
                    if si > 0 {
                        final_skill_indices.push(group.indices[si - 1]);
                    }
                }
            }
        }
    }

    if final_skill_indices.is_empty() {
        println!("\n未选择任何 skill\n");
        return Ok(false);
    }

    // 确认并删除
    let selected_skills: Vec<&Skill> = final_skill_indices.iter().map(|&i| &skills[i]).collect();

    if confirm_delete(&selected_skills)? {
        delete_skills(&selected_skills)?;
        show_complete_message();
        Ok(true)
    } else {
        println!("\n已取消删除操作\n");
        Ok(false)
    }
}

/// 带实时搜索的多选组件
fn searchable_multi_select(
    term: &Term,
    items: &[String],
    search_keys: &[String],
    prompt: &str,
    header: &[String],
) -> Result<Vec<usize>> {
    let mut search = String::new();
    let mut cursor: usize = 0;
    let mut selected: HashSet<usize> = HashSet::new();
    let mut filtered: Vec<usize> = (0..items.len()).collect();

    let (term_height, _) = term.size();
    let page_size = (term_height as usize)
        .saturating_sub(header.len() + 8)
        .max(5)
        .min(20);

    term.hide_cursor()?;
    let _guard = CursorGuard(term);

    loop {
        // 每次重绘前清屏，避免残留内容
        term.clear_screen()?;

        // 渲染头部
        for line in header {
            term.write_line(line)?;
        }

        // 搜索栏
        if search.is_empty() {
            term.write_line(&format!(
                "  {} {}",
                style("🔍 搜索:").cyan(),
                style("(键入关键词实时过滤...)").dim()
            ))?;
        } else {
            term.write_line(&format!(
                "  {} {}  {}",
                style("🔍 搜索:").cyan(),
                style(&search).yellow().bold(),
                style(format!("({}/{})", filtered.len(), items.len())).dim()
            ))?;
        }

        term.write_line("")?;

        // 提示
        term.write_line(&format!(
            "{} {}",
            style("?").green().bold(),
            prompt
        ))?;

        if filtered.is_empty() {
            term.write_line(&format!("  {}", style("无匹配项").yellow()))?;
        } else {
            // 分页
            let start = if cursor >= page_size {
                cursor - page_size + 1
            } else {
                0
            };
            let end = (start + page_size).min(filtered.len());

            if start > 0 {
                term.write_line(&format!(
                    "  {} {}",
                    style("↑").dim(),
                    style(format!("上方还有 {} 项", start)).dim()
                ))?;
            }

            for vi in start..end {
                let idx = filtered[vi];
                let is_cur = vi == cursor;
                let is_sel = selected.contains(&idx);

                let mark = if is_sel {
                    style("◉").green().bold().to_string()
                } else {
                    style("◯").dim().to_string()
                };

                let arrow = if is_cur {
                    style("❯").cyan().bold().to_string()
                } else {
                    " ".to_string()
                };

                let label = if is_cur {
                    style(&items[idx]).bold().to_string()
                } else {
                    items[idx].clone()
                };

                term.write_line(&format!("{} {} {}", arrow, mark, label))?;
            }

            if end < filtered.len() {
                term.write_line(&format!(
                    "  {} {}",
                    style("↓").dim(),
                    style(format!("下方还有 {} 项", filtered.len() - end)).dim()
                ))?;
            }
        }

        term.write_line("")?;
        term.write_line(&format!(
            "  已选 {} 项 | {}选择 {}移动 {}确认 {}{}",
            style(selected.len()).green().bold(),
            style("空格").cyan(),
            style(" ↑↓").cyan(),
            style(" Enter").cyan(),
            style(" Esc").cyan(),
            if search.is_empty() { "退出" } else { "清除搜索" }
        ))?;

        // 读取按键
        match term.read_key()? {
            Key::Char(' ') => {
                if !filtered.is_empty() {
                    let idx = filtered[cursor];
                    if !selected.remove(&idx) {
                        selected.insert(idx);
                    }
                }
            }
            Key::Char(c) if !c.is_control() => {
                search.push(c);
                apply_filter(&search, search_keys, &mut filtered, &mut cursor);
            }
            Key::Backspace => {
                search.pop();
                apply_filter(&search, search_keys, &mut filtered, &mut cursor);
            }
            Key::Escape => {
                if search.is_empty() {
                    return Ok(vec![]);
                }
                search.clear();
                apply_filter(&search, search_keys, &mut filtered, &mut cursor);
            }
            Key::ArrowUp => {
                if cursor > 0 {
                    cursor -= 1;
                }
            }
            Key::ArrowDown => {
                if !filtered.is_empty() && cursor < filtered.len() - 1 {
                    cursor += 1;
                }
            }
            Key::Enter => {
                let mut result: Vec<usize> = selected.into_iter().collect();
                result.sort();
                return Ok(result);
            }
            _ => {}
        }
    }
}

/// 更新过滤结果
fn apply_filter(
    search: &str,
    search_keys: &[String],
    filtered: &mut Vec<usize>,
    cursor: &mut usize,
) {
    let kw = search.to_lowercase();
    if kw.is_empty() {
        *filtered = (0..search_keys.len()).collect();
    } else {
        *filtered = search_keys
            .iter()
            .enumerate()
            .filter(|(_, k)| k.to_lowercase().contains(&kw))
            .map(|(i, _)| i)
            .collect();
    }
    if filtered.is_empty() {
        *cursor = 0;
    } else if *cursor >= filtered.len() {
        *cursor = filtered.len() - 1;
    }
}

/// 显示 skills 列表（非交互模式）
pub fn display_skills(skills: &[Skill]) {
    if skills.is_empty() {
        println!("{}", style("未找到任何 skills").yellow());
        return;
    }

    println!(
        "\n{} 找到 {} 个 skills:\n",
        style("🔍").cyan(),
        style(skills.len()).green().bold()
    );

    for skill in skills {
        println!(
            "  {} {}  {}",
            style(&skill.tool).cyan().bold(),
            style(">").dim(),
            style(&skill.name).white()
        );
        if let Some(desc) = &skill.description {
            println!("     {}", style(desc).dim());
        }
        println!("     {}", style(skill.path.display()).dim().italic());
        println!();
    }
}

/// 确认删除对话框
fn confirm_delete(skills: &[&Skill]) -> Result<bool> {
    if skills.is_empty() {
        return Ok(false);
    }

    println!(
        "\n{} 确定要删除以下 {} 个 skills 吗?\n",
        style("⚠️").yellow(),
        style(skills.len()).red().bold()
    );

    for skill in skills {
        println!(
            "   {} {} > {}",
            style("•").red(),
            style(&skill.tool).cyan(),
            style(&skill.name).white().bold()
        );
        println!("     {}", style(skill.path.display()).dim());
    }

    println!();

    let confirmed = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("确认删除")
        .default(false)
        .interact()?;

    Ok(confirmed)
}

/// 执行删除操作
fn delete_skills(skills: &[&Skill]) -> Result<()> {
    for skill in skills {
        match fs::remove_dir_all(&skill.path) {
            Ok(_) => {
                println!(
                    "{} 已删除: {} > {}",
                    style("✓").green(),
                    style(&skill.tool).cyan(),
                    style(&skill.name).white()
                );
            }
            Err(e) => {
                println!(
                    "{} 删除失败: {} > {} - {}",
                    style("✗").red(),
                    style(&skill.tool).cyan(),
                    style(&skill.name).white(),
                    style(e).red()
                );
            }
        }
    }

    Ok(())
}

/// 显示扫描开始信息
pub fn show_scanning_message(path_count: usize) {
    println!(
        "\n{} 正在扫描 {} 个目录...\n",
        style("🔍").cyan(),
        style(path_count).green()
    );
}

/// 显示完成信息
fn show_complete_message() {
    println!("\n{} 操作完成!\n", style("✨").green());
}
