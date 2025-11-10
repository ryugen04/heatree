package main

import (
	"fmt"
	"os"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/ryugen04/heatree/internal/tui"
)

func main() {
	// コマンドライン引数からパスを取得（デフォルトはカレントディレクトリ）
	path := "."
	if len(os.Args) > 1 {
		path = os.Args[1]
	}

	// TUIモデルの初期化
	m := tui.NewModel(path)

	// Bubble Teaプログラムの起動
	p := tea.NewProgram(m, tea.WithAltScreen())

	if _, err := p.Run(); err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}
}
