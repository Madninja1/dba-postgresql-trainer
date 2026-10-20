use ratatui::{
    Frame,
    layout::{Alignment, Constraint},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::app::App;

use super::common::vertical_chunks;

pub fn render(frame: &mut Frame, app: &App) {
    let strings = app.strings();

    let areas = vertical_chunks(
        frame.area(),
        &[Constraint::Length(3), Constraint::Min(5), Constraint::Length(3)],
    );

    let summary = Paragraph::new(format!(
        "{}: {}",
        strings.resume_title,
        app.unfinished_sessions.len()
    ))
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL));

    if app.unfinished_sessions.is_empty() {
        let empty = Paragraph::new(strings.no_unfinished_quizzes)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title(strings.resume_title)
                    .borders(Borders::ALL),
            );
        frame.render_widget(summary, areas[0]);
        frame.render_widget(empty, areas[1]);
    } else {
        let items = app
            .unfinished_sessions
            .iter()
            .map(|session| {
                ListItem::new(format!(
                    "{} | {}: {} / {} | {}",
                    app.session_title(session),
                    strings.answered,
                    session.answered_questions,
                    session.total_questions,
                    app.session_limit_label(session.limit),
                ))
            })
            .collect::<Vec<_>>();

        let list = List::new(items)
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .block(
                Block::default()
                    .title(strings.continue_quiz)
                    .borders(Borders::ALL),
            );

        let mut state = ListState::default();
        state.select(Some(
            app.resume_selected
                .min(app.unfinished_sessions.len().saturating_sub(1)),
        ));

        frame.render_widget(summary, areas[0]);
        frame.render_stateful_widget(list, areas[1], &mut state);
    }

    let footer = Paragraph::new(strings.footer_resume)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(footer, areas[2]);
}
