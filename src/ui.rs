use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, Tabs, Paragraph},
    text::{Line, Span},
    Frame,
};
use crate::app::{App, SortType};

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3), // Title
                Constraint::Length(3), // Tabs
                Constraint::Min(5),    // Table
            ]
            .as_ref(), // for compatibility with various ratatui versions
        )
        .split(f.area());

    // Header
    let sort_str = match app.sort_type {
        SortType::None => "Default",
        SortType::Name => "Name",
        SortType::ChangePercent => "Change(%)",
    };
    let order_str = if app.sort_type == SortType::None { "" } else if app.sort_descending { "(DESC)" } else { "(ASC)" };
    let help_text = format!("Linux Stock Ticker | 'q': quit, '←/→': tabs, 'c': sort by change(%), 'n': sort by name, 'r': reset sort | Sort: {} {} ", sort_str, order_str);
    
    let title = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Tabs
    let titles: Vec<Line> = app.categories.iter().map(|c| {
        Line::from(Span::styled(&c.name, Style::default().fg(Color::White)))
    }).collect();
    
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Categories"))
        .select(app.active_tab)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    
    f.render_widget(tabs, chunks[1]);

    // Table
    let header_cells = ["Symbol", "Name", "Price", "Change", "Change %"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow)));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    // Filter stocks for the active category
    let active_symbols = &app.categories[app.active_tab].symbols;
    
    let mut filtered_stocks: Vec<_> = app.stocks.iter()
        .filter(|item| active_symbols.contains(&item.symbol))
        .collect();

    match app.sort_type {
        SortType::Name => {
            filtered_stocks.sort_by(|a, b| {
                if app.sort_descending {
                    b.name.cmp(&a.name)
                } else {
                    a.name.cmp(&b.name)
                }
            });
        }
        SortType::ChangePercent => {
            filtered_stocks.sort_by(|a, b| {
                if app.sort_descending {
                    b.change_percent.partial_cmp(&a.change_percent).unwrap_or(std::cmp::Ordering::Equal)
                } else {
                    a.change_percent.partial_cmp(&b.change_percent).unwrap_or(std::cmp::Ordering::Equal)
                }
            });
        }
        SortType::None => {
            filtered_stocks.sort_by_key(|item| active_symbols.iter().position(|s| s == &item.symbol).unwrap_or(999));
        }
    }

    let rows = filtered_stocks.into_iter()
        .map(|item| {
            let color = if item.change_amount > 0.0 {
                Color::Red // Chinese market: Red is up
            } else if item.change_amount < 0.0 {
                Color::Green // Chinese market: Green is down
            } else {
                Color::White
            };

            let cells = vec![
                Cell::from(item.symbol.clone()),
                Cell::from(item.name.clone()),
                Cell::from(format!("{:.2}", item.price)).style(Style::default().fg(color)),
                Cell::from(format!("{:.2}", item.change_amount)).style(Style::default().fg(color)),
                Cell::from(format!("{:.2}%", item.change_percent)).style(Style::default().fg(color)),
            ];
            Row::new(cells).height(1)
        });

    let widths = [
        Constraint::Percentage(15),
        Constraint::Percentage(25),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
    ];
    let t = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(app.categories[app.active_tab].name.clone()));
    
    f.render_widget(t, chunks[2]);
}
