use ratatui::{
    Frame,
    layout::{Alignment, Constraint},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::app::App;

use super::common::{render_message, vertical_chunks};

pub fn render(frame: &mut Frame, app: &App) {
    let tasks = app.practice_tasks_for_selected_category();
    if tasks.is_empty() {
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

    let title = app
        .selected_practice_category
        .as_deref()
        .map(|category| format!("{} — {category}", app.strings().practice_title))
        .unwrap_or_else(|| app.strings().practice_title.to_string());
    let header = Paragraph::new(title)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    let items = tasks
        .iter()
        .map(|task| {
            let progress = app.progress_for_practice_task(task.id);
            let marker = match progress {
                Some(progress) if progress.solved => "✓",
                Some(progress) if progress.is_started() => "•",
                _ => " ",
            };
            let difficulty = app.strings().practice_difficulty(task.difficulty);
            let attempts = progress.map_or(0, |progress| progress.attempts);
            ListItem::new(format!(
                "{marker} {:<8} {}  ·  {} {attempts}",
                difficulty,
                task.title,
                app.strings().practice_attempts,
            ))
        })
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
