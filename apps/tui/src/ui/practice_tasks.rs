use ratatui::{
    Frame,
    layout::{Alignment, Constraint},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::app::App;

use super::common::{render_message, vertical_chunks};

pub fn render(frame: &mut Frame, app: &App) {
    if app.practice_tasks.is_empty() {
        render_message(
            frame,
            app.strings().practice_title,
            app.strings().practice_empty,
            app.strings().footer_back,
        );
        return;
    }

    let areas = vertical_chunks(
        frame.area(),
        &[
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ],
    );

    let header = Paragraph::new(app.strings().practice_title)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    let items = app
        .practice_tasks
        .iter()
        .map(|task| ListItem::new(format!("[{}] {}", task.category, task.title)))
        .collect::<Vec<_>>();

    let list = List::new(items)
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .block(Block::default().borders(Borders::ALL));
    let mut state = ListState::default();
    state.select(Some(app.practice_selected));

    let footer = Paragraph::new(app.strings().footer_practice_tasks)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(header, areas[0]);
    frame.render_stateful_widget(list, areas[1], &mut state);
    frame.render_widget(footer, areas[2]);
}
