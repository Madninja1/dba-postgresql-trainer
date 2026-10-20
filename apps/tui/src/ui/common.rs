use dba_trainer_domain::Topic;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Wrap},
};

const HORIZONTAL_MARGIN: u16 = 2;
const VERTICAL_MARGIN: u16 = 1;
const SECTION_GAP: u16 = 1;

pub fn content_area(area: Rect) -> Rect {
    let horizontal = HORIZONTAL_MARGIN.min(area.width / 2);
    let vertical = VERTICAL_MARGIN.min(area.height / 2);

    Rect::new(
        area.x.saturating_add(horizontal),
        area.y.saturating_add(vertical),
        area.width.saturating_sub(horizontal.saturating_mul(2)),
        area.height.saturating_sub(vertical.saturating_mul(2)),
    )
}

pub fn vertical_chunks(area: Rect, constraints: &[Constraint]) -> Vec<Rect> {
    if constraints.is_empty() {
        return Vec::new();
    }

    let mut spaced = Vec::with_capacity(constraints.len().saturating_mul(2).saturating_sub(1));

    for (index, constraint) in constraints.iter().enumerate() {
        if index > 0 {
            spaced.push(Constraint::Length(SECTION_GAP));
        }

        spaced.push(*constraint);
    }

    Layout::default()
        .direction(Direction::Vertical)
        .constraints(spaced)
        .split(content_area(area))
        .iter()
        .step_by(2)
        .copied()
        .collect()
}

pub fn render_message(frame: &mut Frame, title: &str, message: &str, footer: &str) {
    let areas = vertical_chunks(
        frame.area(),
        &[
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ],
    );

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
