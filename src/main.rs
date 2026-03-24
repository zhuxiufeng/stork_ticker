mod api;
mod app;
mod ui;

use app::{App, Category};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use reqwest::Client;
use std::{error::Error, io, fs, time::{Duration, Instant}};
use tokio::sync::mpsc;

fn load_config() -> Vec<Category> {
    let config_path = "config.json";
    if let Ok(content) = fs::read_to_string(config_path) {
        if let Ok(categories) = serde_json::from_str(&content) {
            return categories;
        }
    }
    
    let defaults = vec![
        Category {
            name: "自选 (All)".to_string(),
            symbols: vec![
                "sh000001".to_string(),
                "sh600519".to_string(),
                "rt_hk00700".to_string(),
                "gb_aapl".to_string(),
                "hf_GC".to_string(),
            ]
        },
        Category {
            name: "A股 (A-Shares)".to_string(),
            symbols: vec![
                "sh000001".to_string(),
                "sz399001".to_string(),
                "sh600519".to_string(),
                "sz000001".to_string(),
            ]
        },
        Category {
            name: "港股 (HK Stocks)".to_string(),
            symbols: vec![
                "rt_hk00700".to_string(),
                "rt_hk03690".to_string(),
                "rt_hk00981".to_string(),
            ]
        },
        Category {
            name: "美股 (US Stocks)".to_string(),
            symbols: vec![
                "gb_aapl".to_string(),
                "gb_msft".to_string(),
                "gb_tsla".to_string(),
                "gb_nvda".to_string(),
            ]
        },
        Category {
            name: "黄金 (Gold)".to_string(),
            symbols: vec![
                "hf_GC".to_string(),
            ]
        },
    ];
    
    if let Ok(json) = serde_json::to_string_pretty(&defaults) {
        let _ = fs::write(config_path, json);
    }
    defaults
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app categories from config
    let categories = load_config();

    let mut app = App::new(categories);
    
    let res = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

async fn run_app(
    terminal: &mut Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
) -> io::Result<()> {
    let tick_rate = Duration::from_millis(100);
    
    // Background task for fetching auto-refreshing data
    let (tx, mut rx) = mpsc::channel(1);
    let all_symbols = app.all_symbols();

    tokio::spawn(async move {
        let client = Client::new();
        let mut unique_symbols = all_symbols;
        unique_symbols.sort();
        unique_symbols.dedup();

        loop {
            if let Ok(data) = api::fetch_stocks(&client, &unique_symbols).await {
                if tx.send(data).await.is_err() {
                    break;
                }
            }
            tokio::time::sleep(Duration::from_secs(3)).await;
        }
    });

    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui::render(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => app.quit(),
                    KeyCode::Char('c') => app.toggle_sort(app::SortType::ChangePercent),
                    KeyCode::Char('n') => app.toggle_sort(app::SortType::Name),
                    KeyCode::Char('r') => app.toggle_sort(app::SortType::None),
                    KeyCode::Right | KeyCode::Tab => app.next_tab(),
                    KeyCode::Left => app.previous_tab(),
                    _ => {}
                }
            }
        }

        while let Ok(data) = rx.try_recv() {
            app.stocks = data;
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }

        if app.should_quit {
            return Ok(());
        }
    }
}
