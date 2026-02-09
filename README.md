# Skills Scanner

一个使用 Rust 编写的 CLI 工具，用于扫描、列出和管理本地各大模型工具的 skills 文件。

## 功能

- 🔍 自动扫描主流 AI 工具的 skills 目录（官方路径优先）
- 📂 支持自定义目录扫描
- 🧭 支持从当前目录向上扫描到 Git 根目录的项目级 skills
- 🌍 跨平台支持 Windows / Linux / macOS
- ✅ 交互式多选界面 (空格选择，Enter 确认)
- 🗑️ 安全删除确认

## 安装

### 通过 npm 安装（推荐）

```bash
# 全局安装
npm install -g skills-scanner

# 或使用 npx 直接运行
npx skills-scanner
```

### 从源码构建

确保已安装 Rust 工具链，然后运行：

```bash
cargo build --release
```

编译后的可 executable 文件位于：

- Windows: `target/release/skills-scanner.exe`
- Linux/macOS: `target/release/skills-scanner`

```bash
# 扫描所有默认目录
skills-scanner

# 扫描指定目录
skills-scanner --path "C:\custom\skills"

# 同时扫描多个目录
skills-scanner --path "C:\dir1" --path "C:\dir2"

# 仅列出 skills，不进入交互模式
skills-scanner --list

# 查看帮助
skills-scanner --help
```

## 支持的目录（默认扫描）

### 用户级目录（跨平台）

| 工具 | 默认目录 |
|------|----------|
| Claude Code | `~/.claude/skills/` |
| OpenAI Codex | `~/.agents/skills/` |
| OpenAI Codex (兼容) | `~/.codex/skills/` |
| Gemini CLI | `~/.gemini/skills/` |
| Windsurf | `~/.codeium/windsurf/skills/` |
| GitHub Copilot | `~/.copilot/skills/` |
| Cursor | `~/.cursor/skills/` |
| Cline | `~/.cline/skills/` |
| OpenCode | `<config>/opencode/skills/` |

`<config>` 使用系统标准配置目录：

- Windows: `%APPDATA%`
- Linux: `~/.config`
- macOS: `~/Library/Application Support`

另外兼容历史目录：
`~/.gemini/antigravity/skills/`、`~/.windsurf/skills/`、`~/.codeium/skills/`、`~/.continue/skills/`、`~/.roo-code/skills/`

### 项目级目录（从当前目录向上到 Git 根）

工具会在每一级目录中尝试发现以下路径：

`./.claude/skills/`、`./.agents/skills/`、`./.github/skills/`、`./.gemini/skills/`、`./.windsurf/skills/`、`./.cursor/skills/`、`./.cline/skills/`、`./.clinerules/skills/`、`./.opencode/skills/`

并扩展扫描常见 Agent Skills 生态目录：

`./.agent/skills/`、`./.augment/skills/`、`./.codebuddy/skills/`、`./.commandcode/skills/`、`./.continue/skills/`、`./.crush/skills/`、`./.factory/skills/`、`./.goose/skills/`、`./.iflow/skills/`、`./.junie/skills/`、`./.kilocode/skills/`、`./.kiro/skills/`、`./.kode/skills/`、`./.mcpjam/skills/`、`./.mux/skills/`、`./.neovate/skills/`、`./.openhands/skills/`、`./.pi/skills/`、`./.pochi/skills/`、`./.qoder/skills/`、`./.qwen/skills/`、`./.roo/skills/`、`./.trae/skills/`、`./.vibe/skills/`、`./.zencoder/skills/`、`./.adal/skills/`、`./.codex/skills/`、`./.roo-code/skills/`

## 交互操作

- `空格` - 选择/取消选择 skill
- `↑/↓` - 上下移动
- `Enter` - 确认选择
- 删除前会弹出确认提示
