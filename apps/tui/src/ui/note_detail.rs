use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Margin},
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap},
};

use crate::app::App;

use super::{common::vertical_chunks, lab_detail::markdown_to_text};

pub fn render(frame: &mut Frame, app: &App) {
    let Some(topic) = app.selected_note_topic.as_ref() else {
        return;
    };

    let areas = vertical_chunks(frame.area(), &[Constraint::Min(5), Constraint::Length(3)]);

    let body = app
        .selected_note_body()
        .unwrap_or(app.strings().notes_missing);
    let content = markdown_to_text(body);
    let content_length = content.lines.len().max(1);
    let viewport_height = areas[0].height.saturating_sub(2) as usize;
    let title = format!(
        "{} · {} · {}. {}",
        topic.course_code.to_uppercase(),
        app.strings().notes_part_label(topic.notes_part),
        topic.topic_number,
        topic.title,
    );

    let paragraph = Paragraph::new(content)
        .wrap(Wrap { trim: false })
        .scroll((app.note_scroll, 0))
        .block(Block::default().title(title).borders(Borders::ALL));

    let footer = Paragraph::new(app.strings().footer_note_detail)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(paragraph, areas[0]);

    if content_length > viewport_height && viewport_height > 0 {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None);
        let mut state = ScrollbarState::new(content_length)
            .viewport_content_length(viewport_height)
            .position(usize::from(app.note_scroll));

        frame.render_stateful_widget(
            scrollbar,
            areas[0].inner(Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut state,
        );
    }

    frame.render_widget(footer, areas[1]);
}
