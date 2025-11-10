# 🔥 heatree

> TUI tool for visualizing code metrics with heatmaps and change frequency

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Go Version](https://img.shields.io/badge/Go-1.21+-00ADD8?logo=go)](https://golang.org)

## 📖 Overview

**heatree** is a terminal-based user interface (TUI) application that helps developers understand codebases quickly by visualizing:

- **Lines of Code** - File and directory sizes with color-coded heatmaps
- **Change Frequency** - Git commit history analysis showing hotspots
- **Tree Structure** - Interactive navigation through project hierarchy

Perfect for:
- 🚀 Onboarding to new projects
- 🔍 Identifying technical debt hotspots
- 📊 Understanding codebase complexity at a glance
- 🎯 Prioritizing refactoring efforts

## 🎯 Key Features

### Interactive TUI Interface

```
┌─────────────────────────────────────────────────────────┐
│ > Source Code Analytics                                 │
│   Visualize code metrics with heatmaps and frequency   │
├─────────────────────────────────────────────────────────┤
│ > Display Options                                       │
│   ○ Lines of Code    ⚡ Change Frequency   ⏱ Per Day   │
└─────────────────────────────────────────────────────────┘

project-root                              Lines    Changes
├─ src/
│  ├─ components/
│  │  ├─ Header.tsx                        127    ████  2.5
│  │  ├─ Footer.tsx                         89    ██    0.8
│  │  ├─ Sidebar.tsx                       234    █████ 4.2
│  │  └─ Button.tsx                         56    ██    1.9
│  ├─ pages/
│  │  ├─ Home.tsx                          456    █████ 6.8
│  │  ├─ About.tsx                         189    ██    1.3
│  │  └─ Contact.tsx                       267    ███   3.1
```

### Core Capabilities

- ✅ **Lines of Code Visualization** - Color-coded by file size (6 tiers)
- ✅ **Change Frequency Analysis** - Git history-based hotspot detection (5 levels)
- ✅ **Time Period Selection** - View changes per day/week/month
- ✅ **Tree Navigation** - Expand/collapse folders, keyboard shortcuts
- ✅ **Filter & Search** - Focus on specific file types or patterns
- ✅ **Export Data** - Generate reports in JSON/CSV format

## 🚀 Quick Start (Planned)

### Installation

```bash
# Via Go install (planned)
go install github.com/ryugen04/heatree@latest
```

### Basic Usage

```bash
# Analyze current directory
heatree .

# Analyze specific repository
heatree /path/to/project

# Show change frequency (default: per day)
heatree --mode frequency .

# Analyze last 30 days
heatree --mode frequency --period 30d .
```

## 🎨 UI Design

Based on modern TUI principles with:
- Dark theme optimized for terminal use
- Color-coded heatmaps (blue → green → yellow → orange → red)
- Smooth navigation with vim-style keybindings
- Real-time filtering and search

See [DESIGN.md](docs/DESIGN.md) for detailed UI specifications.

## 📋 Documentation

- [PROJECT_PLAN.md](docs/PROJECT_PLAN.md) - プロジェクト企画書と要件定義
- [DESIGN.md](docs/DESIGN.md) - UI/UX設計とデータ仕様

## 📋 Roadmap

### Phase 1: MVP (v0.1.0) - 企画中
- [x] Project planning and design
- [ ] Basic tree navigation
- [ ] Lines of code visualization
- [ ] Git integration for change frequency
- [ ] Color-coded heatmaps

### Phase 2: Core Features (v0.2.0)
- [ ] Time period filtering (day/week/month)
- [ ] File type filtering
- [ ] Search functionality
- [ ] Export to JSON/CSV

### Phase 3: Advanced Features (v0.3.0)
- [ ] Complexity metrics integration
- [ ] Custom color schemes
- [ ] Configuration file support
- [ ] Multi-repository comparison

### Phase 4: Polish (v1.0.0)
- [ ] Comprehensive documentation
- [ ] Performance optimizations
- [ ] Cross-platform testing
- [ ] Release automation

## 📄 License

MIT License - see [LICENSE](./LICENSE) file for details.

## 🙏 Acknowledgments

Inspired by:
- [code-forensics](https://github.com/smontanari/code-forensics) - Hotspot analysis
- [CodeScene](https://codescene.io) - Code health metrics
- [lazygit](https://github.com/jesseduffield/lazygit) - Excellent TUI design
- [gitinspector](https://github.com/ejwa/gitinspector) - Git statistics

---

**Built with ❤️ using Go and Bubble Tea**
