use anyhow::Result;
use git2::{Repository, DiffOptions};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct GitAnalyzer {
    repo: Repository,
}

impl GitAnalyzer {
    /// リポジトリを開く
    pub fn new(path: &Path) -> Result<Self> {
        let repo = Repository::open(path)?;
        Ok(Self { repo })
    }

    /// 各ファイルの変更頻度を計算
    pub fn analyze_change_frequency(&self, days: usize) -> Result<HashMap<PathBuf, f64>> {
        let mut changes: HashMap<PathBuf, usize> = HashMap::new();

        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;

        // 最新のコミットから指定日数分を取得
        let mut commit_count = 0;
        let max_commits = days * 10; // 1日10コミット程度を想定

        for oid in revwalk {
            if commit_count >= max_commits {
                break;
            }

            let oid = oid?;
            let commit = self.repo.find_commit(oid)?;

            if commit.parent_count() == 0 {
                continue;
            }

            let parent = commit.parent(0)?;
            let parent_tree = parent.tree()?;
            let commit_tree = commit.tree()?;

            let mut diff_opts = DiffOptions::new();
            let diff = self.repo.diff_tree_to_tree(
                Some(&parent_tree),
                Some(&commit_tree),
                Some(&mut diff_opts),
            )?;

            diff.foreach(
                &mut |delta, _| {
                    if let Some(path) = delta.new_file().path() {
                        let path = path.to_path_buf();
                        *changes.entry(path).or_insert(0) += 1;
                    }
                    true
                },
                None,
                None,
                None,
            )?;

            commit_count += 1;
        }

        // 変更頻度を日次に正規化
        let frequency: HashMap<PathBuf, f64> = changes
            .into_iter()
            .map(|(path, count)| {
                let freq = count as f64 / days as f64;
                (path, freq)
            })
            .collect();

        Ok(frequency)
    }
}
