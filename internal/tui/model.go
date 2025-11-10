package tui

import (
	tea "github.com/charmbracelet/bubbletea"
	"github.com/ryugen04/heatree/internal/analyzer"
	"github.com/ryugen04/heatree/internal/tree"
)

// DisplayMode は表示モードを表す
type DisplayMode int

const (
	DisplayModeLOC DisplayMode = iota // 行数表示モード
	DisplayModeChangeFrequency         // 変更頻度表示モード
)

// Model はBubble TeaのモデルでTUIの状態を保持する
type Model struct {
	ready       bool
	width       int
	height      int
	root        *tree.Node   // ツリーのルートノード
	items       []*tree.Node // 表示するノードのリスト
	cursor      int          // 現在のカーソル位置
	rootPath    string       // 分析対象のルートパス
	err         error        // エラー情報
	displayMode DisplayMode  // 表示モード
}

// NewModel は新しいModelを作成する
func NewModel(rootPath string) Model {
	return Model{
		ready:       false,
		rootPath:    rootPath,
		cursor:      0,
		displayMode: DisplayModeLOC, // デフォルトは行数表示
	}
}

// scanCompleteMsg はスキャン完了を通知するメッセージ
type scanCompleteMsg struct {
	root *tree.Node
	err  error
}

// Init はBubble Teaの初期化メソッド
func (m Model) Init() tea.Cmd {
	return scanDirectory(m.rootPath)
}

// scanDirectory はディレクトリをスキャンするコマンド
func scanDirectory(path string) tea.Cmd {
	return func() tea.Msg {
		// ディレクトリスキャン
		scanner := analyzer.NewScanner(path)
		root, err := scanner.Scan()
		if err != nil {
			return scanCompleteMsg{root: nil, err: err}
		}

		// 行数カウント
		counter := analyzer.NewLOCCounter()
		err = counter.Count(root)
		if err != nil {
			// カウントエラーは無視（部分的な結果を使用）
		}

		// Git履歴解析
		gitAnalyzer := analyzer.NewGitAnalyzer(path)
		err = gitAnalyzer.Analyze(root)
		if err != nil {
			// Git解析エラーは無視（Gitリポジトリでない場合等）
		}

		return scanCompleteMsg{root: root, err: nil}
	}
}
