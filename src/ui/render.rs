use crate::data::FileNode;
use crate::ui::colors::{get_lines_color, get_change_frequency_color};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(frame: &mut Frame, items: &[(usize, FileNode)], selected_index: usize) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // ヘッダー（レジェンド）
            Constraint::Min(0),    // メインコンテンツ
        ])
        .split(frame.area());

    render_legend(frame, chunks[0]);
    render_tree(frame, chunks[1], items, selected_index);
}

fn render_legend(frame: &mut Frame, area: Rect) {
    let legend_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(1)])
        .margin(1)
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
            .block(Block::default().borders(Borders::NONE)),
        legend_chunks[0],
    );

    frame.render_widget(
        Paragraph::new(freq_legend)
            .block(Block::default().borders(Borders::NONE)),
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

fn render_tree(frame: &mut Frame, area: Rect, items: &[(usize, FileNode)], selected_index: usize) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("project-root");

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // ヘッダー行
    let header_area = Rect {
        x: inner.x,
        y: inner.y,
        width: inner.width,
        height: 1,
    };

    let header = create_header_line(inner.width);
    frame.render_widget(Paragraph::new(header), header_area);

    // ツリー行
    let tree_area = Rect {
        x: inner.x,
        y: inner.y + 1,
        width: inner.width,
        height: inner.height.saturating_sub(1),
    };

    let mut lines = Vec::new();
    for (index, (depth, node)) in items.iter().skip(1).enumerate() {
        // rootをスキップ、インデックスを調整
        let actual_index = index + 1;
        let is_selected = actual_index == selected_index;
        let line = create_tree_line(*depth, node, inner.width, is_selected);
        lines.push(line);
    }

    frame.render_widget(Paragraph::new(lines), tree_area);
}

fn create_header_line(width: u16) -> Line<'static> {
    let name_width = (width as f32 * 0.6) as usize;
    let lines_width = 10;
    let changes_width = 10;

    let spans = vec![
        Span::styled(
            format!("{:<name_width$}", "", name_width = name_width),
            Style::default(),
        ),
        Span::styled(
            format!("{:>lines_width$}", "LINES", lines_width = lines_width),
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
        ),
        Span::styled("  ", Style::default()),
        Span::styled(
            format!("{:>changes_width$}", "CHANGES", changes_width = changes_width),
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
        ),
    ];

    Line::from(spans)
}

fn create_tree_line(depth: usize, node: &FileNode, width: u16, is_selected: bool) -> Line<'static> {
    // ツリー罫線を生成（階層構造を視覚化）
    let tree_lines = if depth > 0 {
        let mut lines = String::new();
        for _ in 0..depth - 1 {
            lines.push_str("  ");  // 親階層のインデント
        }
        lines.push_str("│ ");  // 現在の階層の縦線
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

    let name_prefix = format!("{}{}", tree_lines, icon);
    let display_name = if node.is_dir {
        format!("{}/", node.name)
    } else {
        node.name.clone()
    };

    let name_width = (width as f32 * 0.6) as usize;
    let name_text = format!("{}{}", name_prefix, display_name);
    let truncated_name = if name_text.len() > name_width {
        format!("{}...", &name_text[..name_width - 3])
    } else {
        format!("{:<name_width$}", name_text, name_width = name_width)
    };

    let lines_category = node.metrics.lines_category();
    let change_category = node.metrics.change_frequency_category();

    let lines_color = get_lines_color(lines_category);
    let change_color = get_change_frequency_color(change_category);

    // 選択行のスタイル
    let base_style = if is_selected {
        Style::default().bg(Color::DarkGray)
    } else {
        Style::default()
    };

    // ヒートマップバーの幅を計算
    let bar_width = 15;
    let lines_bar = create_heatmap_bar(lines_category, bar_width, lines_color);
    let change_bar = create_heatmap_bar(change_category, bar_width, change_color);

    let spans = vec![
        Span::styled(truncated_name, base_style.fg(Color::White)),
        Span::styled(
            format!(" {:>4} ", node.metrics.lines),
            base_style.fg(lines_color),
        ),
        Span::styled(lines_bar, base_style.fg(lines_color)),
        Span::styled("  ", base_style),
        Span::styled(
            format!(" {:>3.1} ", node.metrics.change_frequency),
            base_style.fg(change_color),
        ),
        Span::styled(change_bar, base_style.fg(change_color)),
    ];

    Line::from(spans)
}

fn create_heatmap_bar(category: usize, max_width: usize, _color: Color) -> String {
    let filled = (category + 1) * (max_width / 6);
    let filled = filled.min(max_width);
    "■".repeat(filled)
}
