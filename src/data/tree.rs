use std::path::PathBuf;
use super::Metrics;

/// ファイルツリーのノード
#[derive(Debug, Clone)]
pub struct FileNode {
    pub name: String,
    #[allow(dead_code)]
    pub path: PathBuf,
    pub is_dir: bool,
    pub children: Vec<FileNode>,
    pub metrics: Metrics,
    pub is_expanded: bool,
}

impl FileNode {
    pub fn new(name: String, path: PathBuf, is_dir: bool) -> Self {
        Self {
            name,
            path,
            is_dir,
            children: Vec::new(),
            metrics: Metrics::default(),
            is_expanded: true,
        }
    }

    /// 子ノードを追加
    pub fn add_child(&mut self, child: FileNode) {
        self.children.push(child);
    }

    /// 子ノードをソート（ディレクトリが先、その後名前順）
    pub fn sort_children(&mut self) {
        self.children.sort_by(|a, b| {
            match (a.is_dir, b.is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.cmp(&b.name),
            }
        });

        for child in &mut self.children {
            child.sort_children();
        }
    }

    /// ツリーを走査して表示用のアイテムリストを生成
    /// 戻り値: (depth, node, is_last_child, parent_continues)
    /// - is_last_child: このノードが親の最後の子かどうか
    /// - parent_continues: 各階層で親が継続しているかのリスト
    pub fn flatten(&self, depth: usize, result: &mut Vec<(usize, FileNode)>) {
        result.push((depth, self.clone()));

        if self.is_expanded && self.is_dir {
            for child in &self.children {
                child.flatten(depth + 1, result);
            }
        }
    }

    /// ツリー罫線情報付きでフラット化
    pub fn flatten_with_lines(&self, depth: usize, is_last: bool, parent_lines: &[bool], result: &mut Vec<(usize, FileNode, bool, Vec<bool>)>) {
        result.push((depth, self.clone(), is_last, parent_lines.to_vec()));

        if self.is_expanded && self.is_dir {
            let child_count = self.children.len();
            let mut new_parent_lines = parent_lines.to_vec();
            new_parent_lines.push(!is_last);

            for (i, child) in self.children.iter().enumerate() {
                let is_last_child = i == child_count - 1;
                child.flatten_with_lines(depth + 1, is_last_child, &new_parent_lines, result);
            }
        }
    }
}
