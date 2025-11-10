package tree

import (
	"path/filepath"
)

// NodeType はノードの種類を表す
type NodeType int

const (
	NodeTypeFile NodeType = iota
	NodeTypeDirectory
)

// Node はツリー構造の1つのノードを表す
type Node struct {
	Name     string    // ファイル/ディレクトリ名
	Path     string    // フルパス
	Type     NodeType  // ファイルかディレクトリか
	Level    int       // 階層レベル（0が最上位）
	Children []*Node   // 子ノード（ディレクトリの場合のみ）
	Parent   *Node     // 親ノード
	Expanded bool      // 展開されているか（ディレクトリの場合のみ）
	Lines    int       // コード行数（将来実装）
	Changes  float64   // 変更頻度（将来実装）
}

// NewNode は新しいノードを作成する
func NewNode(path string, nodeType NodeType, level int) *Node {
	return &Node{
		Name:     filepath.Base(path),
		Path:     path,
		Type:     nodeType,
		Level:    level,
		Children: make([]*Node, 0),
		Expanded: false,
		Lines:    0,
		Changes:  0.0,
	}
}

// IsDirectory はノードがディレクトリかどうかを返す
func (n *Node) IsDirectory() bool {
	return n.Type == NodeTypeDirectory
}

// IsFile はノードがファイルかどうかを返す
func (n *Node) IsFile() bool {
	return n.Type == NodeTypeFile
}

// AddChild は子ノードを追加する
func (n *Node) AddChild(child *Node) {
	child.Parent = n
	n.Children = append(n.Children, child)
}

// Toggle は展開状態を切り替える（ディレクトリの場合のみ）
func (n *Node) Toggle() {
	if n.IsDirectory() {
		n.Expanded = !n.Expanded
	}
}

// ExpandAll は配下のディレクトリをすべて展開する（再帰的）
func (n *Node) ExpandAll() {
	if !n.IsDirectory() {
		return
	}

	n.Expanded = true

	// 子ノードを再帰的に展開
	for _, child := range n.Children {
		if child.IsDirectory() {
			child.ExpandAll()
		}
	}
}

// CollapseAll は配下のディレクトリをすべて折りたたむ（再帰的）
func (n *Node) CollapseAll() {
	if !n.IsDirectory() {
		return
	}

	n.Expanded = false

	// 子ノードを再帰的に折りたたみ
	for _, child := range n.Children {
		if child.IsDirectory() {
			child.CollapseAll()
		}
	}
}
