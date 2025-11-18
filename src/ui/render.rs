use crate::data::FileNode;
use crate::ui::colors::{get_lines_color, get_change_frequency_color};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Table, Row, Cell},
    Frame,
};

pub fn render(frame: &mut Frame, items: &[(usize, FileNode, bool, Vec<bool>)], selected_index: usize) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // ヘッダー（レジェンド）- 2行分に拡大
            Constraint::Min(0),    // メインコンテンツ
        ])
        .split(frame.area());

    render_legend(frame, chunks[0]);
    render_tree(frame, chunks[1], items, selected_index);
}

fn render_legend(frame: &mut Frame, area: Rect) {
    let legend_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Length(2)])
        .margin(0)
        .split(area);

    // Lines of Code レジェンド
    let lines_legend = create_legend_line(
        "Lines of Code:",
        &[
            (0, "<50"),
            (1, "50-100"),
            (2, "100-200"),
            (3, "200-500"),
            (4, "500-1K"),
            (5, "1K+"),
        ],
        true,
    );

    // Change Frequency レジェンド
    let freq_legend = create_legend_line(
        "Change Frequency/day:",
        &[
            (0, "<1.7"),
            (1, "1.7-3.4"),
            (2, "3.4-5.2"),
            (3, "5.2-6.9"),
            (4, "6.9+"),
        ],
        false,
    );

    frame.render_widget(
        Paragraph::new(lines_legend)
            .block(Block::default().borders(Borders::NONE))
            .style(Style::default().bg(Color::Black)),
        legend_chunks[0],
    );

    frame.render_widget(
        Paragraph::new(freq_legend)
            .block(Block::default().borders(Borders::NONE))
            .style(Style::default().bg(Color::Black)),
        legend_chunks[1],
    );
}

fn create_legend_line(label: &str, items: &[(usize, &str)], is_lines: bool) -> Line<'static> {
    let mut spans = vec![Span::styled(
        format!("{}  ", label),
        Style::default().fg(Color::White),
    )];

    for (category, text) in items {
        let color = if is_lines {
            get_lines_color(*category)
        } else {
            get_change_frequency_color(*category)
        };

        spans.push(Span::styled("■ ", Style::default().fg(color)));
        spans.push(Span::styled(
            format!("{}  ", text),
            Style::default().fg(Color::Gray),
        ));
    }

    Line::from(spans)
}

fn render_tree(frame: &mut Frame, area: Rect, items: &[(usize, FileNode, bool, Vec<bool>)], selected_index: usize) {
    // ヘッダー行を作成
    let header = Row::new(vec![
        Cell::from(""),
        Cell::from(format!("{:>20} ", "LINES")).style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        Cell::from(format!("{:>20} ", "CHANGES")).style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
    ]);

    // データ行を作成
    let mut rows = Vec::new();
    for (index, (depth, node, is_last, parent_lines)) in items.iter().skip(1).enumerate() {
        let actual_index = index + 1;
        let is_selected = actual_index == selected_index;

        let row = create_table_row(*depth, node, *is_last, parent_lines, is_selected);
        rows.push(row);
    }

    // 列幅の設定
    let widths = [
        Constraint::Percentage(50),  // Name列（可変）
        Constraint::Length(40),       // LINES列（固定40文字）
        Constraint::Length(40),       // CHANGES列（固定40文字）
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("project-root"))
        .column_spacing(0); // スペースなし（各セル内でパディングを制御）

    frame.render_widget(table, area);
}

/// Tableの行を作成
fn create_table_row(depth: usize, node: &FileNode, is_last: bool, parent_lines: &[bool], is_selected: bool) -> Row<'static> {
    // Name列の内容を作成
    let name_cell = create_name_cell(depth, node, is_last, parent_lines, is_selected);

    // LINES列の内容を作成
    let lines_cell = create_lines_cell(node, is_selected);

    // CHANGES列の内容を作成
    let changes_cell = create_changes_cell(node, is_selected);

    let style = if is_selected {
        Style::default().bg(Color::DarkGray)
    } else {
        Style::default()
    };

    Row::new(vec![name_cell, lines_cell, changes_cell])
        .style(style)
        .height(1)
}

/// Name列のセルを作成
fn create_name_cell(depth: usize, node: &FileNode, is_last: bool, parent_lines: &[bool], _is_selected: bool) -> Cell<'static> {
    let tree_lines = if depth > 0 {
        let mut lines = String::new();

        // 親階層の継続線を描画
        for &continues in parent_lines {
            if continues {
                lines.push_str("│ ");
            } else {
                lines.push_str("  ");
            }
        }

        // 現在階層の分岐線を描画
        if is_last {
            lines.push_str("└─");
        } else {
            lines.push_str("├─");
        }

        lines
    } else {
        String::new()
    };

    let icon = if node.is_dir {
        if node.is_expanded {
            "[▼] "
        } else {
            "[▶] "
        }
    } else {
        "[ ] "
    };

    let display_name = if node.is_dir {
        format!("{}/", node.name)
    } else {
        node.name.clone()
    };

    let content = format!("{}{}{}", tree_lines, icon, display_name);

    Cell::from(content).style(Style::default().fg(Color::White))
}

/// LINES列のセルを作成（グラフバー付き）
fn create_lines_cell(node: &FileNode, _is_selected: bool) -> Cell<'static> {
    let category = node.metrics.lines_category();
    let base_color = get_lines_color(category);

    // カテゴリーの色に基づいた暗めの背景色
    let bar_bg = match base_color {
        Color::Rgb(r, g, b) => {
            Color::Rgb(
                (r as f32 * 0.35) as u8,
                (g as f32 * 0.35) as u8,
                (b as f32 * 0.35) as u8,
            )
        },
        _ => Color::Rgb(20, 30, 40),
    };

    // カテゴリーに基づいてインジケーターの透明度を調整（数値が小さいほど薄く）
    let indicator_color = match base_color {
        Color::Rgb(r, g, b) => {
            // カテゴリー0(最小)は背景色、カテゴリー5(最大)は元の色
            let opacity = 0.1 + (category as f32 * 0.18); // 0から5で0.1→1.0
            let bg_r = 15u8;
            let bg_g = 15u8;
            let bg_b = 15u8;
            Color::Rgb(
                (bg_r as f32 * (1.0 - opacity) + r as f32 * opacity) as u8,
                (bg_g as f32 * (1.0 - opacity) + g as f32 * opacity) as u8,
                (bg_b as f32 * (1.0 - opacity) + b as f32 * opacity) as u8,
            )
        },
        _ => base_color,
    };

    let bar_count = (category + 1) * 2;
    let bar_str = "■".repeat(bar_count);

    // 色インジケーター + 数値
    let indicator = "█ ";
    let value_str = format!("{:>6}", node.metrics.lines);

    // グラフバー部分（固定幅のボックス）
    // 最大バー幅（12個の■ = 24文字幅）+ 余裕を持たせて36文字幅
    let total_bg_width = 36; // 背景エリア全体の固定幅

    // バーの文字幅を計算（■は全角なので2文字幅）
    let bar_width = bar_count * 2;

    // 背景エリアを固定幅の文字列として構築
    let bg_content = if bar_width < total_bg_width {
        format!(" {}{}", bar_str, " ".repeat(total_bg_width - 1 - bar_width))
    } else {
        format!(" {}", bar_str)
    };

    let line = Line::from(vec![
        Span::styled(indicator, Style::default().fg(indicator_color)),
        Span::styled(value_str, Style::default().fg(base_color)),
        Span::raw(" "),
        Span::styled(bg_content, Style::default().fg(base_color).bg(bar_bg)),
    ]);

    Cell::from(line)
}

/// CHANGES列のセルを作成（グラフバー付き）
fn create_changes_cell(node: &FileNode, _is_selected: bool) -> Cell<'static> {
    let category = node.metrics.change_frequency_category();
    let base_color = get_change_frequency_color(category);

    // カテゴリーの色に基づいた暗めの背景色
    let bar_bg = match base_color {
        Color::Rgb(r, g, b) => {
            Color::Rgb(
                (r as f32 * 0.35) as u8,
                (g as f32 * 0.35) as u8,
                (b as f32 * 0.35) as u8,
            )
        },
        _ => Color::Rgb(20, 30, 40),
    };

    // カテゴリーに基づいてインジケーターの透明度を調整（数値が小さいほど薄く）
    let indicator_color = match base_color {
        Color::Rgb(r, g, b) => {
            // カテゴリー0(最小)は背景色、カテゴリー4(最大)は元の色
            let opacity = 0.1 + (category as f32 * 0.225); // 0から4で0.1→1.0
            let bg_r = 15u8;
            let bg_g = 15u8;
            let bg_b = 15u8;
            Color::Rgb(
                (bg_r as f32 * (1.0 - opacity) + r as f32 * opacity) as u8,
                (bg_g as f32 * (1.0 - opacity) + g as f32 * opacity) as u8,
                (bg_b as f32 * (1.0 - opacity) + b as f32 * opacity) as u8,
            )
        },
        _ => base_color,
    };

    let bar_count = (category + 1) * 2;
    let bar_str = "■".repeat(bar_count);

    // 色インジケーター + 数値
    let indicator = "█ ";
    let value_str = format!("{:>6.1}", node.metrics.change_frequency);

    // グラフバー部分（固定幅のボックス）
    // 最大バー幅（12個の■ = 24文字幅）+ 余裕を持たせて36文字幅
    let total_bg_width = 36; // 背景エリア全体の固定幅

    // バーの文字幅を計算（■は全角なので2文字幅）
    let bar_width = bar_count * 2;

    // 背景エリアを固定幅の文字列として構築
    let bg_content = if bar_width < total_bg_width {
        format!(" {}{}", bar_str, " ".repeat(total_bg_width - 1 - bar_width))
    } else {
        format!(" {}", bar_str)
    };

    let line = Line::from(vec![
        Span::styled(indicator, Style::default().fg(indicator_color)),
        Span::styled(value_str, Style::default().fg(base_color)),
        Span::raw(" "),
        Span::styled(bg_content, Style::default().fg(base_color).bg(bar_bg)),
    ]);

    Cell::from(line)
}


