use std::path::PathBuf;
use std::fs;

/// 表示一个 skill 的数据结构
#[derive(Debug, Clone)]
pub struct Skill {
    /// Skill 名称
    pub name: String,
    /// 所属工具名称 (Cursor, Claude, Gemini 等)
    pub tool: String,
    /// Skill 目录的完整路径
    pub path: PathBuf,
    /// 从 SKILL.md 提取的描述
    pub description: Option<String>,
}

impl Skill {
    /// 从目录路径创建 Skill
    pub fn from_path(path: PathBuf, tool: &str) -> Option<Self> {
        let name = path.file_name()?.to_string_lossy().to_string();
        let description = Self::extract_description(&path);
        
        Some(Self {
            name,
            tool: tool.to_string(),
            path,
            description,
        })
    }
    
    /// 从 SKILL.md 文件提取描述
    fn extract_description(skill_path: &PathBuf) -> Option<String> {
        let skill_md = skill_path.join("SKILL.md");
        if !skill_md.exists() {
            return None;
        }
        
        let content = fs::read_to_string(&skill_md).ok()?;
        
        // 尝试从 YAML frontmatter 提取 description
        if content.starts_with("---") {
            let parts: Vec<&str> = content.splitn(3, "---").collect();
            if parts.len() >= 3 {
                for line in parts[1].lines() {
                    let line = line.trim();
                    if line.starts_with("description:") {
                        let desc = line.strip_prefix("description:")?.trim();
                        // 移除可能的引号
                        let desc = desc.trim_matches('"').trim_matches('\'');
                        return Some(desc.to_string());
                    }
                }
            }
        }
        
        // 如果没有 frontmatter，取第一个非空行作为描述
        content.lines()
            .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
            .next()
            .map(|s| s.trim().to_string())
    }
    
    /// 格式化显示
    pub fn display_name(&self) -> String {
        format!("{:<12} > {}", self.tool, self.name)
    }
    
    /// 获取描述，如果没有则返回默认值
    pub fn display_description(&self) -> &str {
        self.description.as_deref().unwrap_or("No description")
    }
}
