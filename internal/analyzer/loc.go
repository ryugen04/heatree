package analyzer

import (
	"bufio"
	"os"
	"strings"

	"github.com/ryugen04/heatree/internal/tree"
)

// LOCCounter はコード行数をカウントする
type LOCCounter struct {
	SkipEmpty    bool // 空行をスキップするか
	SkipComments bool // コメント行をスキップするか（将来実装）
}

// NewLOCCounter は新しいLOCCounterを作成する
func NewLOCCounter() *LOCCounter {
	return &LOCCounter{
		SkipEmpty:    true,
		SkipComments: false, // 現在はコメントも含める
	}
}

// Count はノードツリーの全ファイルの行数をカウントする
func (c *LOCCounter) Count(root *tree.Node) error {
	return c.countRecursive(root)
}

// countRecursive は再帰的にノードの行数をカウントする
func (c *LOCCounter) countRecursive(node *tree.Node) error {
	if node.IsFile() {
		// ファイルの行数をカウント
		lines, err := c.countFile(node.Path)
		if err != nil {
			// エラーは無視して0とする（バイナリファイル、権限エラー等）
			node.Lines = 0
		} else {
			node.Lines = lines
		}
	} else if node.IsDirectory() {
		// ディレクトリの場合、子要素を再帰的に処理
		totalLines := 0
		for _, child := range node.Children {
			err := c.countRecursive(child)
			if err != nil {
				// エラーは無視して続行
				continue
			}
			totalLines += child.Lines
		}
		node.Lines = totalLines
	}

	return nil
}

// countFile はファイルの行数をカウントする
func (c *LOCCounter) countFile(filePath string) (int, error) {
	file, err := os.Open(filePath)
	if err != nil {
		return 0, err
	}
	defer file.Close()

	lineCount := 0
	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()

		// 空行をスキップする場合
		if c.SkipEmpty && strings.TrimSpace(line) == "" {
			continue
		}

		lineCount++
	}

	if err := scanner.Err(); err != nil {
		return 0, err
	}

	return lineCount, nil
}

// IsBinaryFile はファイルがバイナリかどうかを簡易的に判定する（将来実装）
func (c *LOCCounter) isBinaryFile(filePath string) bool {
	// 簡易実装: 拡張子でチェック
	binaryExtensions := []string{
		".exe", ".dll", ".so", ".dylib",
		".jpg", ".jpeg", ".png", ".gif", ".bmp",
		".zip", ".tar", ".gz", ".bz2",
		".pdf", ".doc", ".docx",
		".class", ".o", ".a",
	}

	for _, ext := range binaryExtensions {
		if strings.HasSuffix(strings.ToLower(filePath), ext) {
			return true
		}
	}

	return false
}
