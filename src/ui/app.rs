use crate::data::FileNode;

pub struct App {
    pub root: FileNode,
    pub should_quit: bool,
    pub selected_index: usize,
}

impl App {
    pub fn new(root: FileNode) -> Self {
        Self {
            root,
            should_quit: false,
            selected_index: 0,
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// ツリーを平坦化して表示用のリストを取得
    pub fn get_flat_tree(&self) -> Vec<(usize, FileNode)> {
        let mut result = Vec::new();
        self.root.flatten(0, &mut result);
        result
    }

    /// ツリー罫線情報付きで平坦化
    pub fn get_flat_tree_with_lines(&self) -> Vec<(usize, FileNode, bool, Vec<bool>)> {
        let mut result = Vec::new();
        self.root.flatten_with_lines(0, true, &[], &mut result);
        result
    }

    /// 選択行を上に移動
    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    /// 選択行を下に移動
    pub fn move_down(&mut self) {
        let items = self.get_flat_tree();
        if self.selected_index < items.len().saturating_sub(1) {
            self.selected_index += 1;
        }
    }

    /// 選択されたノードを展開/折りたたみ
    pub fn toggle_selected(&mut self) {
        let items = self.get_flat_tree();
        if self.selected_index >= items.len() {
            return;
        }

        let selected_path = items[self.selected_index].1.path.clone();
        self.toggle_node_by_path(&selected_path);
    }

    /// パスを指定してノードを展開/折りたたみ
    fn toggle_node_by_path(&mut self, path: &std::path::Path) {
        Self::toggle_recursive(&mut self.root, path);
    }

    fn toggle_recursive(node: &mut FileNode, target_path: &std::path::Path) -> bool {
        if node.path == target_path {
            if node.is_dir {
                node.is_expanded = !node.is_expanded;
            }
            return true;
        }

        for child in &mut node.children {
            if Self::toggle_recursive(child, target_path) {
                return true;
            }
        }

        false
    }
}
