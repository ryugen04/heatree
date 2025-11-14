/// ファイル/ディレクトリのメトリクス
#[derive(Debug, Clone, Default)]
pub struct Metrics {
    /// コード行数
    pub lines: usize,
    /// 変更頻度（日次）
    pub change_frequency: f64,
}

impl Metrics {
    pub fn new(lines: usize, change_frequency: f64) -> Self {
        Self {
            lines,
            change_frequency,
        }
    }

    /// 行数のカテゴリを取得（ヒートマップ用）
    pub fn lines_category(&self) -> usize {
        match self.lines {
            0..=49 => 0,
            50..=99 => 1,
            100..=199 => 2,
            200..=499 => 3,
            500..=999 => 4,
            _ => 5,
        }
    }

    /// 変更頻度のカテゴリを取得（ヒートマップ用）
    pub fn change_frequency_category(&self) -> usize {
        if self.change_frequency < 1.7 {
            0
        } else if self.change_frequency < 3.4 {
            1
        } else if self.change_frequency < 5.2 {
            2
        } else if self.change_frequency < 6.9 {
            3
        } else {
            4
        }
    }
}
