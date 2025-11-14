use ratatui::style::Color;

/// 行数カテゴリに対応する色を取得
pub fn get_lines_color(category: usize) -> Color {
    match category {
        0 => Color::Rgb(100, 100, 100),  // <50: グレー
        1 => Color::Rgb(50, 150, 200),   // 50-100: ライトブルー
        2 => Color::Rgb(100, 200, 100),  // 100-200: グリーン
        3 => Color::Rgb(200, 200, 100),  // 200-500: イエロー
        4 => Color::Rgb(255, 165, 0),    // 500-1K: オレンジ
        _ => Color::Rgb(200, 50, 50),    // 1K+: レッド
    }
}

/// 変更頻度カテゴリに対応する色を取得
pub fn get_change_frequency_color(category: usize) -> Color {
    match category {
        0 => Color::Rgb(50, 100, 150),   // <1.7: ダークブルー
        1 => Color::Rgb(50, 150, 200),   // 1.7-3.4: シアン
        2 => Color::Rgb(100, 200, 100),  // 3.4-5.2: グリーン
        3 => Color::Rgb(255, 200, 50),   // 5.2-6.9: イエロー
        _ => Color::Rgb(220, 50, 50),    // 6.9+: レッド
    }
}
