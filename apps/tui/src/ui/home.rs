use ratatui::{
    Frame,
    layout::{Alignment, Constraint},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::app::{App, HOME_ITEMS};

use super::common::vertical_chunks;

pub fn render(frame: &mut Frame, app: &App) {
    let areas = vertical_chunks(
        frame.area(),
        &[
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(6),
            Constraint::Length(3),
        ],
    );

    let header = Paragraph::new("PostgreSQL DBA Trainer")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    let rank = app
        .activity_stats
        .as_ref()
        .map(|stats| app.strings().progress_level(stats.level()))
        .unwrap_or("—");

    let rank = Paragraph::new(format!("{}: {rank}", app.strings().rank))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    let items = HOME_ITEMS
        .iter()
        .map(|item| ListItem::new(item.label(app.strings(), app.language)))
        .collect::<Vec<_>>();

    let menu = List::new(items)
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .block(
            Block::default()
                .title(app.strings().main_menu)
                .borders(Borders::ALL),
        );

    let mut menu_state = ListState::default();
    menu_state.select(Some(app.home_selected));

    let footer = Paragraph::new(app.strings().footer_menu)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(header, areas[0]);
    frame.render_widget(rank, areas[1]);
    frame.render_stateful_widget(menu, areas[2], &mut menu_state);
    frame.render_widget(footer, areas[3]);
}
