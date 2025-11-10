package tree

// FlattenTree はツリー構造を平坦なリストに変換する（表示用）
// 展開されているノードのみを含む
func FlattenTree(root *Node) []*Node {
	result := make([]*Node, 0)
	flattenRecursive(root, &result)
	return result
}

// flattenRecursive は再帰的にノードを平坦化する
func flattenRecursive(node *Node, result *[]*Node) {
	*result = append(*result, node)

	// ディレクトリが展開されている場合のみ子要素を追加
	if node.IsDirectory() && node.Expanded {
		for _, child := range node.Children {
			flattenRecursive(child, result)
		}
	}
}

// GetVisibleNodes はツリーから表示すべきノードのリストを返す
// ルートノードは除外する（ルート自体は表示しない）
func GetVisibleNodes(root *Node) []*Node {
	result := make([]*Node, 0)

	// ルートが展開されている場合、その子要素から開始
	if root.IsDirectory() && root.Expanded {
		for _, child := range root.Children {
			flattenRecursive(child, &result)
		}
	}

	return result
}
