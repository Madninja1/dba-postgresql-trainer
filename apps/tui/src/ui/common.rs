use dba_trainer_domain::Topic;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn render_message(frame: &mut Frame, title: &str, message: &str, footer: &str) {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let header = Paragraph::new(title)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    let body = Paragraph::new(message)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL));

    let footer = Paragraph::new(footer)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(header, areas[0]);
    frame.render_widget(body, areas[1]);
    frame.render_widget(footer, areas[2]);
}

pub fn topic_display_number(topics: &[&Topic], index: usize) -> String {
    let topic = topics[index];

    let group_size = topics
        .iter()
        .filter(|candidate| candidate.notes_part == topic.notes_part)
        .count();

    if group_size == 1 {
        return topic.notes_part.to_string();
    }

    let subtopic_number = topics
        .iter()
        .take(index + 1)
        .filter(|candidate| candidate.notes_part == topic.notes_part)
        .count();

    format!("{}.{}", topic.notes_part, subtopic_number)
}
