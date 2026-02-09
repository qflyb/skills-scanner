use std::collections::HashSet;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::skill::Skill;

/// 相对路径目录配置
#[derive(Debug, Clone, Copy)]
struct RelativeSkillPathConfig {
    /// 工具名称
    tool_name: &'static str,
    /// 相对路径片段
    path_parts: &'static [&'static str],
}

/// 用户目录下的主流技能路径（全平台：Windows/Linux/macOS）
const USER_HOME_SKILL_PATH_CONFIGS: &[RelativeSkillPathConfig] = &[
    RelativeSkillPathConfig {
        tool_name: "Claude Code",
        path_parts: &[".claude", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "OpenAI Codex",
        path_parts: &[".agents", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "OpenAI Codex (Legacy)",
        path_parts: &[".codex", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "Gemini CLI",
        path_parts: &[".gemini", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "Windsurf",
        path_parts: &[".codeium", "windsurf", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "GitHub Copilot",
        path_parts: &[".copilot", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "Cursor",
        path_parts: &[".cursor", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "Cline",
        path_parts: &[".cline", "skills"],
    },
    // 兼容历史目录
    RelativeSkillPathConfig {
        tool_name: "Gemini Antigravity (Legacy)",
        path_parts: &[".gemini", "antigravity", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "Windsurf (Legacy)",
        path_parts: &[".windsurf", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "Codeium (Legacy)",
        path_parts: &[".codeium", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "Continue (Legacy)",
        path_parts: &[".continue", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "Roo Code (Legacy)",
        path_parts: &[".roo-code", "skills"],
    },
];

/// 配置目录下的主流技能路径（遵循平台标准配置路径）
const USER_CONFIG_SKILL_PATH_CONFIGS: &[RelativeSkillPathConfig] = &[
    RelativeSkillPathConfig {
        tool_name: "OpenCode",
        path_parts: &["opencode", "skills"],
    },
];

/// 工作区目录下的技能路径（从当前目录向上查找到 git 根）
const WORKSPACE_SKILL_PATH_CONFIGS: &[RelativeSkillPathConfig] = &[
    // 官方路径
    RelativeSkillPathConfig {
        tool_name: "Claude Code (Project)",
        path_parts: &[".claude", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "OpenAI Codex (Project)",
        path_parts: &[".agents", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "GitHub Copilot (Project)",
        path_parts: &[".github", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "Gemini CLI (Project)",
        path_parts: &[".gemini", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "Windsurf (Project)",
        path_parts: &[".windsurf", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "Cursor (Project)",
        path_parts: &[".cursor", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "Cline (Project)",
        path_parts: &[".cline", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "Cline Compatibility (Project)",
        path_parts: &[".clinerules", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "OpenCode (Project)",
        path_parts: &[".opencode", "skills"],
    },
    // 扩展目录（Agent Skills 生态常见目录）
    RelativeSkillPathConfig {
        tool_name: "Antigravity (Project)",
        path_parts: &[".agent", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "Augment (Project)",
        path_parts: &[".augment", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "Codebuddy (Project)",
        path_parts: &[".codebuddy", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "CommandCode (Project)",
        path_parts: &[".commandcode", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "Continue (Project)",
        path_parts: &[".continue", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "Crush (Project)",
        path_parts: &[".crush", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "Factory (Project)",
        path_parts: &[".factory", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "Goose (Project)",
        path_parts: &[".goose", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "iFlow (Project)",
        path_parts: &[".iflow", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "Junie (Project)",
        path_parts: &[".junie", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "KiloCode (Project)",
        path_parts: &[".kilocode", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "Kiro (Project)",
        path_parts: &[".kiro", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "Kode (Project)",
        path_parts: &[".kode", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "MCP Jam (Project)",
        path_parts: &[".mcpjam", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "Mux (Project)",
        path_parts: &[".mux", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "Neovate (Project)",
        path_parts: &[".neovate", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "OpenHands (Project)",
        path_parts: &[".openhands", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "Pi (Project)",
        path_parts: &[".pi", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "Pochi (Project)",
        path_parts: &[".pochi", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "Qoder (Project)",
        path_parts: &[".qoder", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "Qwen (Project)",
        path_parts: &[".qwen", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "Roo (Project)",
        path_parts: &[".roo", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "Trae (Project)",
        path_parts: &[".trae", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "Vibe (Project)",
        path_parts: &[".vibe", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "Zencoder (Project)",
        path_parts: &[".zencoder", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "Adal (Project)",
        path_parts: &[".adal", "skills"],
    },
    // 兼容历史目录
    RelativeSkillPathConfig {
        tool_name: "OpenAI Codex (Legacy Project)",
        path_parts: &[".codex", "skills"],
    },
    RelativeSkillPathConfig {
        tool_name: "Roo Code (Legacy Project)",
        path_parts: &[".roo-code", "skills"],
    },
];

#[cfg(not(windows))]
const UNIX_SYSTEM_SKILL_PATHS: &[(&str, &str)] = &[("OpenAI Codex (Admin)", "/etc/codex/skills")];

/// Skills 扫描器
pub struct Scanner {
    /// 扫描路径列表: (工具名, 路径)
    paths: Vec<(String, PathBuf)>,
}

impl Scanner {
    /// 创建扫描器，自动添加默认扫描路径
    pub fn new() -> Self {
        let mut scanner = Self { paths: Vec::new() };
        scanner.add_default_paths();
        scanner
    }
    
    /// 添加默认的工具 skills 目录
    fn add_default_paths(&mut self) {
        let mut seen_directory_paths: HashSet<PathBuf> = HashSet::new();
        self.add_user_home_paths(&mut seen_directory_paths);
        self.add_user_config_paths(&mut seen_directory_paths);
        self.add_workspace_paths(&mut seen_directory_paths);
        self.add_system_paths(&mut seen_directory_paths);
    }

    /// 添加用户目录下的 skills 路径
    fn add_user_home_paths(&mut self, seen_directory_paths: &mut HashSet<PathBuf>) {
        if let Some(home_directory_path) = dirs::home_dir() {
            for path_config in USER_HOME_SKILL_PATH_CONFIGS {
                debug_assert!(!path_config.path_parts.is_empty());
                let candidate_path =
                    join_path_parts(&home_directory_path, path_config.path_parts);
                self.add_existing_directory(
                    path_config.tool_name,
                    candidate_path,
                    seen_directory_paths,
                );
            }
        }
    }

    /// 添加平台配置目录下的 skills 路径
    fn add_user_config_paths(&mut self, seen_directory_paths: &mut HashSet<PathBuf>) {
        if let Some(config_directory_path) = dirs::config_dir() {
            for path_config in USER_CONFIG_SKILL_PATH_CONFIGS {
                debug_assert!(!path_config.path_parts.is_empty());
                let candidate_path =
                    join_path_parts(&config_directory_path, path_config.path_parts);
                self.add_existing_directory(
                    path_config.tool_name,
                    candidate_path,
                    seen_directory_paths,
                );
            }
        }
    }

    /// 添加工作区目录下的 skills 路径（当前目录向上扫描）
    fn add_workspace_paths(&mut self, seen_directory_paths: &mut HashSet<PathBuf>) {
        for workspace_directory_path in workspace_search_paths() {
            for path_config in WORKSPACE_SKILL_PATH_CONFIGS {
                debug_assert!(!path_config.path_parts.is_empty());
                let candidate_path =
                    join_path_parts(&workspace_directory_path, path_config.path_parts);
                self.add_existing_directory(
                    path_config.tool_name,
                    candidate_path,
                    seen_directory_paths,
                );
            }
        }
    }

    /// 添加系统级 skills 路径
    fn add_system_paths(&mut self, _seen_directory_paths: &mut HashSet<PathBuf>) {
        #[cfg(not(windows))]
        for (tool_name, absolute_path) in UNIX_SYSTEM_SKILL_PATHS {
            let candidate_path = PathBuf::from(absolute_path);
            self.add_existing_directory(tool_name, candidate_path, _seen_directory_paths);
        }
    }

    /// 仅在目录存在时加入扫描列表，并做去重
    fn add_existing_directory(
        &mut self,
        tool_name: &str,
        directory_path: PathBuf,
        seen_directory_paths: &mut HashSet<PathBuf>,
    ) {
        if !directory_path.is_dir() {
            return;
        }
        if seen_directory_paths.insert(directory_path.clone()) {
            self.paths.push((tool_name.to_string(), directory_path));
        }
    }
    
    /// 添加自定义扫描路径
    pub fn add_custom_path(&mut self, path: PathBuf) {
        let is_duplicate_path = self
            .paths
            .iter()
            .any(|(_, existing_path)| *existing_path == path);
        if path.is_dir() && !is_duplicate_path {
            self.paths.push(("Custom".to_string(), path));
        }
    }
    
    /// 执行扫描，返回所有找到的 skills
    pub fn scan(&self) -> Vec<Skill> {
        let mut discovered_skills = Vec::new();
        
        for (tool_name, base_directory_path) in &self.paths {
            // 支持 base_path 本身就是一个 skill 目录
            self.collect_skill_directory(base_directory_path, tool_name, &mut discovered_skills);

            // 兼容一层子目录布局：<skills>/<skill-name>/SKILL.md
            for entry in WalkDir::new(base_directory_path)
                .min_depth(1)
                .max_depth(1)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let skill_directory_path = entry.path();
                if skill_directory_path.is_dir() {
                    self.collect_skill_directory(
                        skill_directory_path,
                        tool_name,
                        &mut discovered_skills,
                    );
                }
            }
        }
        
        discovered_skills
    }

    /// 将符合条件的目录转换为 Skill
    fn collect_skill_directory(
        &self,
        skill_directory_path: &Path,
        tool_name: &str,
        discovered_skills: &mut Vec<Skill>,
    ) {
        if !skill_directory_path.join("SKILL.md").is_file() {
            return;
        }
        if let Some(discovered_skill) = Skill::from_path(skill_directory_path.to_path_buf(), tool_name)
        {
            discovered_skills.push(discovered_skill);
        }
    }
    
    /// 获取扫描路径数量
    pub fn path_count(&self) -> usize {
        self.paths.len()
    }
}

impl Default for Scanner {
    fn default() -> Self {
        Self::new()
    }
}

/// 将路径片段拼接到基础路径上
fn join_path_parts(base_path: &Path, path_parts: &[&str]) -> PathBuf {
    debug_assert!(!path_parts.is_empty());
    let mut full_path = base_path.to_path_buf();
    for path_part in path_parts {
        full_path.push(path_part);
    }
    full_path
}

/// 获取从当前目录到 git 根目录的路径链（包含当前目录和 git 根）
fn workspace_search_paths() -> Vec<PathBuf> {
    let current_directory_path = match std::env::current_dir() {
        Ok(path) => path,
        Err(_) => return Vec::new(),
    };

    let mut ancestor_paths = Vec::new();
    for ancestor_path in current_directory_path.ancestors() {
        ancestor_paths.push(ancestor_path.to_path_buf());
        if ancestor_path.join(".git").exists() {
            break;
        }
    }

    ancestor_paths
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn create_temp_directory(suffix: &str) -> PathBuf {
        let timestamp_nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time is earlier than UNIX_EPOCH")
            .as_nanos();
        let temp_directory_path = std::env::temp_dir().join(format!(
            "skills-scanner-test-{}-{}-{}",
            suffix,
            std::process::id(),
            timestamp_nanos
        ));
        fs::create_dir_all(&temp_directory_path)
            .expect("failed to create temporary test directory");
        temp_directory_path
    }

    fn write_skill_markdown(skill_directory_path: &Path) {
        let skill_markdown_path = skill_directory_path.join("SKILL.md");
        fs::write(
            skill_markdown_path,
            "---\nname: test-skill\ndescription: test\n---\n# test\n",
        )
        .expect("failed to write SKILL.md");
    }

    #[test]
    fn scan_detects_base_and_child_skill_directories() {
        let temp_directory_path = create_temp_directory("scan-layout");
        let nested_skill_directory_path = temp_directory_path.join("nested-skill");
        let non_skill_directory_path = temp_directory_path.join("not-a-skill");

        fs::create_dir_all(&nested_skill_directory_path)
            .expect("failed to create nested skill directory");
        fs::create_dir_all(&non_skill_directory_path)
            .expect("failed to create non skill directory");

        write_skill_markdown(&temp_directory_path);
        write_skill_markdown(&nested_skill_directory_path);

        let scanner = Scanner {
            paths: vec![("TestTool".to_string(), temp_directory_path.clone())],
        };
        let found_skills = scanner.scan();

        assert_eq!(found_skills.len(), 2);
        let has_nested_skill = found_skills
            .iter()
            .any(|skill| skill.name == "nested-skill" && skill.tool == "TestTool");
        assert!(has_nested_skill);

        fs::remove_dir_all(temp_directory_path).expect("failed to clean up temp directory");
    }

    #[test]
    fn add_custom_path_ignores_duplicates_and_missing_directories() {
        let temp_directory_path = create_temp_directory("custom-path");
        let missing_directory_path = temp_directory_path.join("missing-directory");

        let mut scanner = Scanner { paths: Vec::new() };
        scanner.add_custom_path(temp_directory_path.clone());
        scanner.add_custom_path(temp_directory_path.clone());
        scanner.add_custom_path(missing_directory_path);

        assert_eq!(scanner.path_count(), 1);

        fs::remove_dir_all(temp_directory_path).expect("failed to clean up temp directory");
    }
}
