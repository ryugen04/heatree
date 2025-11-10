package tui

import (
	"fmt"
	"strings"

	"github.com/charmbracelet/lipgloss"
	"github.com/ryugen04/heatree/internal/tree"
)

// View はBubble Teaの描画メソッドでUIを表示する
func (m Model) View() string {
	if !m.ready {
		return "Loading..."
	}

	// エラーがある場合
	if m.err != nil {
		return fmt.Sprintf("Error: %v\n\nPress 'q' to quit", m.err)
	}

	// ツリーがまだロードされていない場合
	if m.root == nil {
		return "Loading directory..."
	}

	var b strings.Builder

	// ヘッダー
	b.WriteString(m.renderHeader())
	b.WriteString("\n\n")

	// レジェンド
	b.WriteString(m.renderLegend())
	b.WriteString("\n\n")

	// ツリー表示
	b.WriteString(m.renderTree())
	b.WriteString("\n\n")

	// フッター
	b.WriteString(m.renderFooter())

	return b.String()
}

// renderHeader はヘッダーを描画する
func (m Model) renderHeader() string {
	titleStyle := lipgloss.NewStyle().
		Bold(true).
		Foreground(lipgloss.Color("#0EA5E9"))

	pathStyle := lipgloss.NewStyle().
		Foreground(lipgloss.Color("#c0caf5"))

	title := titleStyle.Render("🔥 heatree")
	path := pathStyle.Render(fmt.Sprintf("  %s", m.root.Path))

	return title + path
}

// renderLegend はレジェンドを描画する
func (m Model) renderLegend() string {
	labelStyle := lipgloss.NewStyle().
		Foreground(lipgloss.Color("#c0caf5"))

	var label string
	var legend string

	if m.displayMode == DisplayModeLOC {
		label = labelStyle.Render("Lines of Code: ")
		legend = GetLOCLegend()
	} else {
		label = labelStyle.Render("Change Frequency: ")
		legend = GetChangeFrequencyLegend()
	}

	return label + legend
}

// renderTree はツリーを描画する
func (m Model) renderTree() string {
	if len(m.items) == 0 {
		return "No files found"
	}

	var b strings.Builder

	for i, node := range m.items {
		line := m.renderTreeLine(node, i == m.cursor)
		b.WriteString(line)
		b.WriteString("\n")
	}

	return b.String()
}

// renderTreeLine はツリーの1行を描画する
func (m Model) renderTreeLine(node *tree.Node, selected bool) string {
	// インデント
	indent := strings.Repeat("  ", node.Level-1)

	// 展開状態アイコンとファイル/ディレクトリアイコン
	var icon string
	if node.IsDirectory() {
		if node.Expanded {
			icon = "▼ 📁"
		} else {
			icon = "▶ 📁"
		}
	} else {
		icon = "  📄"
	}

	// 名前
	name := node.Name

	// モードに応じた表示
	var valueStr string
	var heatBar string
	var valueColor string

	if m.displayMode == DisplayModeLOC {
		// 行数表示モード
		valueStr = fmt.Sprintf("%6d", node.Lines)
		heatBar = RenderHeatBar(node.Lines, 10)
		valueColor = GetLOCColor(node.Lines)
	} else {
		// 変更頻度表示モード
		valueStr = fmt.Sprintf("%6.2f", node.Changes)
		heatBar = RenderChangeFrequencyBar(node.Changes, 10)
		valueColor = GetChangeFrequencyColor(node.Changes)
	}

	// スタイル適用
	var style lipgloss.Style
	var valueStyle lipgloss.Style

	if selected {
		style = lipgloss.NewStyle().
			Background(lipgloss.Color("#414868")).
			Foreground(lipgloss.Color("#c0caf5"))
		valueStyle = lipgloss.NewStyle().
			Background(lipgloss.Color("#414868")).
			Foreground(lipgloss.Color(valueColor))
	} else {
		style = lipgloss.NewStyle().
			Foreground(lipgloss.Color("#c0caf5"))
		valueStyle = lipgloss.NewStyle().
			Foreground(lipgloss.Color(valueColor))
	}

	namepart := fmt.Sprintf("%s%s %s", indent, icon, name)

	// 名前部分、ヒートマップバー、値を組み合わせ
	return style.Render(namepart) + "  " + heatBar + "  " + valueStyle.Render(valueStr)
}

// renderFooter はフッターを描画する
func (m Model) renderFooter() string {
	helpStyle := lipgloss.NewStyle().
		Foreground(lipgloss.Color("#414868"))

	help := "j/k: Navigate  o: Toggle  O: Expand All  Tab/1/2: Switch Mode  q: Quit"

	// 現在のモード表示
	var modeStr string
	if m.displayMode == DisplayModeLOC {
		modeStr = "Lines of Code"
	} else {
		modeStr = "Change Frequency"
	}

	stats := fmt.Sprintf("Items: %d  Cursor: %d  Mode: %s", len(m.items), m.cursor, modeStr)

	return helpStyle.Render(help + "  |  " + stats)
}
