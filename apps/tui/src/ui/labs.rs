use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::app::App;

use super::common::render_message;

pub fn render(frame: &mut Frame, app: &App) {
    let labs = app.labs_for_selected_course();

    if labs.is_empty() {
        render_message(
            frame,
            app.strings().labs_title,
            app.strings().labs_empty,
            app.strings().footer_back,
        );
        return;
    }

    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title = match app.selected_lab_course.as_deref() {
        Some(course) => format!("{} → {}", app.strings().labs_title, course.to_uppercase()),
        None => app.strings().labs_title.to_string(),
    };

    let header = Paragraph::new(title)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    let items = labs
        .iter()
        .enumerate()
        .map(|(index, lab)| {
            let style = if index == app.lab_selected {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };

            ListItem::new(format!("{}. {}", lab.lab_number, lab.title)).style(style)
        })
        .collect::<Vec<_>>();

    let list = List::new(items).block(Block::default().borders(Borders::ALL));
    let mut state = ListState::default();
    state.select(Some(app.lab_selected));

    let footer = Paragraph::new(app.strings().footer_labs)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(header, areas[0]);
    frame.render_stateful_widget(list, areas[1], &mut state);
    frame.render_widget(footer, areas[2]);
}
