package analyzer

import (
	"os/exec"
	"path/filepath"
	"strings"
	"time"

	"github.com/ryugen04/heatree/internal/tree"
)

// GitAnalyzer はGit履歴を解析する
type GitAnalyzer struct {
	RepoPath string        // リポジトリのパス
	Period   time.Duration // 分析期間（デフォルト: 30日）
}

// NewGitAnalyzer は新しいGitAnalyzerを作成する
func NewGitAnalyzer(repoPath string) *GitAnalyzer {
	return &GitAnalyzer{
		RepoPath: repoPath,
		Period:   30 * 24 * time.Hour, // デフォルト30日
	}
}

// Analyze はGit履歴を解析してノードツリーに変更頻度を設定する
func (g *GitAnalyzer) Analyze(root *tree.Node) error {
	// Gitリポジトリかどうかチェック
	if !g.isGitRepository() {
		// Gitリポジトリでない場合はエラーを返さず、変更頻度を0のままにする
		return nil
	}

	// ファイルごとの変更回数を取得
	changeCount, err := g.getFileChangeCounts()
	if err != nil {
		return err
	}

	// 分析期間の日数を計算
	days := g.Period.Hours() / 24

	// ノードツリーに変更頻度を設定
	g.setChangeFrequency(root, changeCount, days)

	return nil
}

// isGitRepository はGitリポジトリかどうかチェックする
func (g *GitAnalyzer) isGitRepository() bool {
	cmd := exec.Command("git", "-C", g.RepoPath, "rev-parse", "--git-dir")
	err := cmd.Run()
	return err == nil
}

// getFileChangeCounts は指定期間内のファイルごとの変更回数を取得する
func (g *GitAnalyzer) getFileChangeCounts() (map[string]int, error) {
	// 指定期間前の日付を計算
	since := time.Now().Add(-g.Period)
	sinceStr := since.Format("2006-01-02")

	// git log コマンドを実行
	// --name-only: 変更されたファイル名のみ表示
	// --pretty=format: コミットメッセージは表示しない
	cmd := exec.Command(
		"git", "-C", g.RepoPath,
		"log",
		"--since="+sinceStr,
		"--name-only",
		"--pretty=format:",
	)

	output, err := cmd.Output()
	if err != nil {
		return nil, err
	}

	// ファイルごとの変更回数をカウント
	changeCount := make(map[string]int)
	lines := strings.Split(string(output), "\n")

	for _, line := range lines {
		line = strings.TrimSpace(line)
		if line == "" {
			continue
		}

		// 絶対パスに変換
		absPath := filepath.Join(g.RepoPath, line)
		changeCount[absPath]++
	}

	return changeCount, nil
}

// setChangeFrequency はノードツリーに変更頻度を設定する（再帰的）
func (g *GitAnalyzer) setChangeFrequency(node *tree.Node, changeCount map[string]int, days float64) {
	if node.IsFile() {
		// ファイルの変更頻度を計算
		count := changeCount[node.Path]
		if days > 0 {
			node.Changes = float64(count) / days
		} else {
			node.Changes = 0
		}
	} else if node.IsDirectory() {
		// ディレクトリの場合、子要素を再帰的に処理
		totalChanges := 0.0
		fileCount := 0

		for _, child := range node.Children {
			g.setChangeFrequency(child, changeCount, days)

			// ファイルのみカウント（ディレクトリの平均を計算するため）
			if child.IsFile() {
				totalChanges += child.Changes
				fileCount++
			}
		}

		// ディレクトリの変更頻度は子ファイルの平均
		if fileCount > 0 {
			node.Changes = totalChanges / float64(fileCount)
		} else {
			node.Changes = 0
		}
	}
}

// SetPeriod は分析期間を設定する
func (g *GitAnalyzer) SetPeriod(days int) {
	g.Period = time.Duration(days) * 24 * time.Hour
}

// GetPeriodDays は分析期間を日数で取得する
func (g *GitAnalyzer) GetPeriodDays() int {
	return int(g.Period.Hours() / 24)
}
