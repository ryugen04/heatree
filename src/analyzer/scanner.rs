use crate::data::{FileNode, Metrics};
use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// ディレクトリをスキャンしてファイルツリーを構築
pub fn scan_directory(
    root: &Path,
    frequency_map: &HashMap<PathBuf, f64>,
) -> Result<FileNode> {
    let mut root_node = FileNode::new(
        root.file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string(),
        root.to_path_buf(),
        true,
    );

    build_tree(&mut root_node, root, root, frequency_map)?;
    root_node.sort_children();

    Ok(root_node)
}

fn build_tree(
    node: &mut FileNode,
    root: &Path,
    current: &Path,
    frequency_map: &HashMap<PathBuf, f64>,
) -> Result<()> {
    if !current.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        // .gitなどの隠しファイル/ディレクトリをスキップ
        if name.starts_with('.') {
            continue;
        }

        let is_dir = path.is_dir();
        let mut child = FileNode::new(name, path.clone(), is_dir);

        if is_dir {
            build_tree(&mut child, root, &path, frequency_map)?;
            // ディレクトリの場合、子要素のメトリクスを集計
            child.metrics = aggregate_metrics(&child);
        } else {
            // ファイルの場合、行数と変更頻度を取得
            let lines = count_lines(&path).unwrap_or(0);
            let relative_path = path.strip_prefix(root).unwrap_or(&path);
            let frequency = frequency_map
                .get(relative_path)
                .copied()
                .unwrap_or(0.0);

            child.metrics = Metrics::new(lines, frequency);
        }

        node.add_child(child);
    }

    Ok(())
}

fn aggregate_metrics(node: &FileNode) -> Metrics {
    let mut total_lines = 0;
    let mut total_frequency = 0.0;
    let mut file_count = 0;

    for child in &node.children {
        total_lines += child.metrics.lines;
        if !child.is_dir {
            total_frequency += child.metrics.change_frequency;
            file_count += 1;
        } else {
            // 再帰的にディレクトリの平均も考慮
            total_frequency += child.metrics.change_frequency;
            if child.metrics.lines > 0 {
                file_count += 1;
            }
        }
    }

    let avg_frequency = if file_count > 0 {
        total_frequency / file_count as f64
    } else {
        0.0
    };

    Metrics::new(total_lines, avg_frequency)
}

fn count_lines(path: &Path) -> Result<usize> {
    let content = fs::read_to_string(path)?;
    Ok(content.lines().count())
}
