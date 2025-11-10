package tui

import (
	"strings"

	"github.com/charmbracelet/lipgloss"
)

// ColorLevel は色の段階を表す
type ColorLevel int

const (
	ColorLevelVeryLow ColorLevel = iota
	ColorLevelLow
	ColorLevelMedium
	ColorLevelHigh
	ColorLevelVeryHigh
	ColorLevelExtreme
)

// LOCColorScheme は行数に基づく6段階のカラースキーム
var LOCColorScheme = map[ColorLevel]string{
	ColorLevelVeryLow:  "#0EA5E9", // Blue - < 50行
	ColorLevelLow:      "#06B6D4", // Cyan - 50-100行
	ColorLevelMedium:   "#10B981", // Green - 100-200行
	ColorLevelHigh:     "#F59E0B", // Yellow - 200-500行
	ColorLevelVeryHigh: "#F97316", // Orange - 500-1000行
	ColorLevelExtreme:  "#EF4444", // Red - 1000行以上
}

// GetLOCColor は行数に応じた色を返す
func GetLOCColor(lines int) string {
	level := GetLOCLevel(lines)
	return LOCColorScheme[level]
}

// GetLOCLevel は行数に応じたカラーレベルを返す
func GetLOCLevel(lines int) ColorLevel {
	if lines < 50 {
		return ColorLevelVeryLow
	} else if lines < 100 {
		return ColorLevelLow
	} else if lines < 200 {
		return ColorLevelMedium
	} else if lines < 500 {
		return ColorLevelHigh
	} else if lines < 1000 {
		return ColorLevelVeryHigh
	}
	return ColorLevelExtreme
}

// RenderHeatBar は行数に応じたヒートマップバーを描画する
func RenderHeatBar(lines int, maxWidth int) string {
	if lines == 0 {
		return strings.Repeat(" ", maxWidth)
	}

	color := GetLOCColor(lines)
	style := lipgloss.NewStyle().Foreground(lipgloss.Color(color))

	// バーの長さを計算（最大幅に対する割合）
	// 対数スケールを使用してより見やすく
	barLength := calculateBarLength(lines, maxWidth)

	bar := strings.Repeat("█", barLength)
	padding := strings.Repeat(" ", maxWidth-barLength)

	return style.Render(bar) + padding
}

// calculateBarLength はバーの長さを計算する
func calculateBarLength(lines int, maxWidth int) int {
	if lines == 0 {
		return 0
	}

	// 対数スケールを使用（見やすさのため）
	// 10行で1ブロック、100行で2ブロック、1000行で3ブロックなど
	var barLength int

	if lines < 50 {
		barLength = 1
	} else if lines < 100 {
		barLength = 2
	} else if lines < 200 {
		barLength = 3
	} else if lines < 500 {
		barLength = 5
	} else if lines < 1000 {
		barLength = 7
	} else {
		barLength = 10
	}

	// 最大幅を超えないように
	if barLength > maxWidth {
		barLength = maxWidth
	}

	return barLength
}

// GetLOCLegend は行数のレジェンドを返す
func GetLOCLegend() string {
	var b strings.Builder

	items := []struct {
		level ColorLevel
		label string
	}{
		{ColorLevelVeryLow, "< 50"},
		{ColorLevelLow, "50-100"},
		{ColorLevelMedium, "100-200"},
		{ColorLevelHigh, "200-500"},
		{ColorLevelVeryHigh, "500-1000"},
		{ColorLevelExtreme, "1000+"},
	}

	for i, item := range items {
		color := LOCColorScheme[item.level]
		style := lipgloss.NewStyle().Foreground(lipgloss.Color(color))

		b.WriteString(style.Render("█ " + item.label))

		if i < len(items)-1 {
			b.WriteString("  ")
		}
	}

	return b.String()
}

// ChangeFrequencyColorScheme は変更頻度に基づく5段階のカラースキーム
var ChangeFrequencyColorScheme = map[ColorLevel]string{
	ColorLevelVeryLow:  "#0EA5E9", // Blue - < 1.7/day
	ColorLevelLow:      "#06B6D4", // Cyan - 1.7-3.4/day
	ColorLevelMedium:   "#10B981", // Green - 3.4-5.2/day
	ColorLevelHigh:     "#F59E0B", // Yellow - 5.2-6.9/day
	ColorLevelVeryHigh: "#EF4444", // Red - 6.9+/day
}

// GetChangeFrequencyColor は変更頻度に応じた色を返す
func GetChangeFrequencyColor(changes float64) string {
	level := GetChangeFrequencyLevel(changes)
	return ChangeFrequencyColorScheme[level]
}

// GetChangeFrequencyLevel は変更頻度に応じたカラーレベルを返す
func GetChangeFrequencyLevel(changes float64) ColorLevel {
	if changes < 1.7 {
		return ColorLevelVeryLow
	} else if changes < 3.4 {
		return ColorLevelLow
	} else if changes < 5.2 {
		return ColorLevelMedium
	} else if changes < 6.9 {
		return ColorLevelHigh
	}
	return ColorLevelVeryHigh
}

// RenderChangeFrequencyBar は変更頻度に応じたヒートマップバーを描画する
func RenderChangeFrequencyBar(changes float64, maxWidth int) string {
	if changes == 0 {
		return strings.Repeat(" ", maxWidth)
	}

	color := GetChangeFrequencyColor(changes)
	style := lipgloss.NewStyle().Foreground(lipgloss.Color(color))

	// バーの長さを計算
	barLength := calculateChangeFrequencyBarLength(changes, maxWidth)

	bar := strings.Repeat("█", barLength)
	padding := strings.Repeat(" ", maxWidth-barLength)

	return style.Render(bar) + padding
}

// calculateChangeFrequencyBarLength はバーの長さを計算する
func calculateChangeFrequencyBarLength(changes float64, maxWidth int) int {
	if changes == 0 {
		return 0
	}

	var barLength int

	if changes < 1.7 {
		barLength = 1
	} else if changes < 3.4 {
		barLength = 3
	} else if changes < 5.2 {
		barLength = 5
	} else if changes < 6.9 {
		barLength = 7
	} else {
		barLength = 10
	}

	// 最大幅を超えないように
	if barLength > maxWidth {
		barLength = maxWidth
	}

	return barLength
}

// GetChangeFrequencyLegend は変更頻度のレジェンドを返す
func GetChangeFrequencyLegend() string {
	var b strings.Builder

	items := []struct {
		level ColorLevel
		label string
	}{
		{ColorLevelVeryLow, "< 1.7/day"},
		{ColorLevelLow, "1.7-3.4/day"},
		{ColorLevelMedium, "3.4-5.2/day"},
		{ColorLevelHigh, "5.2-6.9/day"},
		{ColorLevelVeryHigh, "6.9+/day"},
	}

	for i, item := range items {
		color := ChangeFrequencyColorScheme[item.level]
		style := lipgloss.NewStyle().Foreground(lipgloss.Color(color))

		b.WriteString(style.Render("█ " + item.label))

		if i < len(items)-1 {
			b.WriteString("  ")
		}
	}

	return b.String()
}
