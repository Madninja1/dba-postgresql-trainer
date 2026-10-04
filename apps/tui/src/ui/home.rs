use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::{App, HOME_ITEMS};

pub fn render(frame: &mut Frame, app: &App) {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(6),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let header = Paragraph::new("PostgreSQL DBA Trainer")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    let items = HOME_ITEMS
        .iter()
        .enumerate()
        .map(|(index, item)| {
            let style = if index == app.home_selected {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };

            ListItem::new(item.label()).style(style)
        })
        .collect::<Vec<_>>();

    let menu = List::new(items).block(Block::default().title("Главное меню").borders(Borders::ALL));

    let footer = Paragraph::new("↑/↓ или j/k — выбор | Enter — открыть | q — выход")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(header, areas[0]);
    frame.render_widget(menu, areas[1]);
    frame.render_widget(footer, areas[2]);
}
