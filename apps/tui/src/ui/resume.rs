use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::App;

pub fn render(frame: &mut Frame, app: &App) {
    let Some(session) = app.resume_session.as_ref() else {
        return;
    };

    let strings = app.strings();

    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),
            Constraint::Min(4),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let next_question = session
        .answered_questions
        .saturating_add(1)
        .min(session.total_questions);

    let progress = Paragraph::new(format!(
        "{}\n{}: {} / {}\n{}: {}",
        strings.resume_found,
        strings.answered,
        session.answered_questions,
        session.total_questions,
        strings.next_question,
        next_question,
    ))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .title(strings.resume_title)
            .borders(Borders::ALL),
    );

    let labels = [strings.continue_quiz, strings.cancel_quiz];

    let items = labels
        .iter()
        .enumerate()
        .map(|(index, label)| {
            let style = if index == app.decision_selected {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };

            ListItem::new(*label).style(style)
        })
        .collect::<Vec<_>>();

    let actions =
        List::new(items).block(Block::default().title(strings.action).borders(Borders::ALL));

    let footer = Paragraph::new(strings.footer_resume)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(progress, areas[0]);
    frame.render_widget(actions, areas[1]);
    frame.render_widget(footer, areas[2]);
}
