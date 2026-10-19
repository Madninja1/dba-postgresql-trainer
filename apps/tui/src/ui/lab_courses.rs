use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::App;

use super::common::render_message;

pub fn render(frame: &mut Frame, app: &App) {
    let courses = app.lab_course_codes();

    if courses.is_empty() {
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
        .constraints([Constraint::Min(5), Constraint::Length(3)])
        .split(frame.area());

    let items = courses
        .iter()
        .enumerate()
        .map(|(index, course)| {
            let style = if index == app.lab_course_selected {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };

            ListItem::new(course.to_uppercase()).style(style)
        })
        .collect::<Vec<_>>();

    let list = List::new(items).block(
        Block::default()
            .title(app.strings().choose_lab_course)
            .borders(Borders::ALL),
    );

    let footer = Paragraph::new(app.strings().footer_labs)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(list, areas[0]);
    frame.render_widget(footer, areas[1]);
}
