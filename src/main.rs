mod analyzer;
mod data;
mod ui;

use analyzer::{scan_directory, GitAnalyzer};
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::env;
use std::io;
use std::path::PathBuf;
use ui::{render::render, App};

fn main() -> Result<()> {
    // コマンドライン引数からパスを取得（デフォルトはカレントディレクトリ）
    let path = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| env::current_dir().unwrap());

    // Git解析
    let analyzer = GitAnalyzer::new(&path)?;
    let frequency_map = analyzer.analyze_change_frequency(30)?; // 過去30日分

    // ディレクトリスキャン
    let root = scan_directory(&path, &frequency_map)?;

    // TUIセットアップ
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // アプリケーション実行
    let mut app = App::new(root);
    let res = run_app(&mut terminal, &mut app);

    // TUIクリーンアップ
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()> {
    loop {
        let items = app.get_flat_tree();
        terminal.draw(|f| render(f, &items))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        app.quit();
                    }
                    _ => {}
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
