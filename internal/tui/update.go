package tui

import (
	tea "github.com/charmbracelet/bubbletea"
	"github.com/ryugen04/heatree/internal/tree"
)

// Update はBubble Teaの更新メソッドでイベントを処理する
func (m Model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.String() {
		case "ctrl+c", "q":
			return m, tea.Quit

		case "j", "down":
			// 下に移動
			if m.cursor < len(m.items)-1 {
				m.cursor++
			}

		case "k", "up":
			// 上に移動
			if m.cursor > 0 {
				m.cursor--
			}

		case "o":
			// o キーで展開/折りたたみのトグル（1階層のみ）
			if len(m.items) > 0 && m.cursor < len(m.items) {
				currentNode := m.items[m.cursor]
				if currentNode.IsDirectory() {
					currentNode.Toggle()
					// ツリーを再構築
					m.items = tree.GetVisibleNodes(m.root)
				}
			}

		case "O":
			// O キー（Shift+o）で配下すべてを展開
			if len(m.items) > 0 && m.cursor < len(m.items) {
				currentNode := m.items[m.cursor]
				if currentNode.IsDirectory() {
					currentNode.ExpandAll()
					// ツリーを再構築
					m.items = tree.GetVisibleNodes(m.root)
				}
			}

		case "tab", "1", "2":
			// Tabキーまたは1/2キーでモード切り替え
			if msg.String() == "1" {
				m.displayMode = DisplayModeLOC
			} else if msg.String() == "2" {
				m.displayMode = DisplayModeChangeFrequency
			} else {
				// Tabキーでトグル
				if m.displayMode == DisplayModeLOC {
					m.displayMode = DisplayModeChangeFrequency
				} else {
					m.displayMode = DisplayModeLOC
				}
			}
		}

	case tea.WindowSizeMsg:
		m.width = msg.Width
		m.height = msg.Height
		m.ready = true

	case scanCompleteMsg:
		// スキャン完了
		m.err = msg.err
		if msg.err == nil && msg.root != nil {
			m.root = msg.root
			m.items = tree.GetVisibleNodes(m.root)
		}
	}

	return m, nil
}
