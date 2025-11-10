package analyzer

import (
	"os"
	"path/filepath"
	"strings"

	"github.com/ryugen04/heatree/internal/tree"
)

// Scanner はディレクトリをスキャンしてツリー構造を構築する
type Scanner struct {
	RootPath        string
	ExcludePatterns []string
}

// NewScanner は新しいScannerを作成する
func NewScanner(rootPath string) *Scanner {
	return &Scanner{
		RootPath: rootPath,
		ExcludePatterns: []string{
			".git",
			"node_modules",
			"vendor",
			".idea",
			".vscode",
			"__pycache__",
			".pytest_cache",
			"dist",
			"build",
			"target",
		},
	}
}

// Scan はディレクトリをスキャンしてルートノードを返す
func (s *Scanner) Scan() (*tree.Node, error) {
	absPath, err := filepath.Abs(s.RootPath)
	if err != nil {
		return nil, err
	}

	root := tree.NewNode(absPath, tree.NodeTypeDirectory, 0)
	root.Expanded = true // ルートは最初から展開

	err = s.scanDirectory(root, absPath, 0)
	if err != nil {
		return nil, err
	}

	return root, nil
}

// scanDirectory は再帰的にディレクトリをスキャンする
func (s *Scanner) scanDirectory(parent *tree.Node, path string, level int) error {
	entries, err := os.ReadDir(path)
	if err != nil {
		return err
	}

	for _, entry := range entries {
		// 除外パターンのチェック
		if s.shouldExclude(entry.Name()) {
			continue
		}

		fullPath := filepath.Join(path, entry.Name())

		var node *tree.Node
		if entry.IsDir() {
			node = tree.NewNode(fullPath, tree.NodeTypeDirectory, level+1)
			parent.AddChild(node)

			// 再帰的にサブディレクトリをスキャン
			err = s.scanDirectory(node, fullPath, level+1)
			if err != nil {
				// エラーは無視して続行（パーミッションエラー等）
				continue
			}
		} else {
			node = tree.NewNode(fullPath, tree.NodeTypeFile, level+1)
			parent.AddChild(node)
		}
	}

	return nil
}

// shouldExclude はファイル/ディレクトリを除外すべきか判定する
func (s *Scanner) shouldExclude(name string) bool {
	// 隠しファイル/ディレクトリ（先頭が.）は除外パターンにない限りスキップ
	if strings.HasPrefix(name, ".") {
		for _, pattern := range s.ExcludePatterns {
			if pattern == name {
				return true
			}
		}
		// .gitなど明示的に除外パターンにあるもの以外の隠しファイルも除外
		return true
	}

	for _, pattern := range s.ExcludePatterns {
		if name == pattern {
			return true
		}
	}

	return false
}
