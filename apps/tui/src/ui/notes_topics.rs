use ratatui::{
    Frame,
    layout::{Alignment, Constraint},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::app::App;

use super::common::{render_message, topic_display_number, vertical_chunks};

pub fn render(frame: &mut Frame, app: &App) {
    let topics = app.notes_topics_for_selected_course();

    if topics.is_empty() {
        render_message(
            frame,
            app.strings().notes_title,
            app.strings().notes_empty,
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

    let title = match app.selected_notes_course.as_deref() {
        Some(course_code) => format!(
            "{} → {}",
            app.strings().notes_title,
            course_code.to_uppercase(),
        ),
        None => app.strings().notes_title.to_string(),
    };

    let header = Paragraph::new(title)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    let mut items = Vec::new();
    let mut selected_row = 0usize;
    let mut previous_part = None;

    for (index, topic) in topics.iter().enumerate() {
        if previous_part != Some(topic.notes_part) {
            items.push(
                ListItem::new(format!(
                    "── {} ──",
                    app.strings().notes_part_label(topic.notes_part)
                ))
                .style(Style::default().add_modifier(Modifier::BOLD | Modifier::DIM)),
            );
            previous_part = Some(topic.notes_part);
        }

        if index == app.notes_topic_selected {
            selected_row = items.len();
        }

        let number = topic_display_number(&topics, index);
        let has_note = app.notes.iter().any(|note| {
            note.topic.course == topic.course_code && note.topic.slug == topic.slug
        });
        let suffix = if has_note { "" } else { "  ·  —" };

        items.push(ListItem::new(format!("{number}. {}{suffix}", topic.title)));
    }

    let list = List::new(items)
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .block(Block::default().borders(Borders::ALL));

    let mut state = ListState::default();
    state.select(Some(selected_row));

    let footer = Paragraph::new(app.strings().footer_notes)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(header, areas[0]);
    frame.render_stateful_widget(list, areas[1], &mut state);
    frame.render_widget(footer, areas[2]);
}
