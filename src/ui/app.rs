use crate::data::FileNode;

pub struct App {
    pub root: FileNode,
    pub should_quit: bool,
}

impl App {
    pub fn new(root: FileNode) -> Self {
        Self {
            root,
            should_quit: false,
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
}
