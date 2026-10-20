use ratatui::{
    Frame,
    layout::{Alignment, Constraint},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
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
            Constraint::Length(6),
            Constraint::Min(6),
            Constraint::Length(3),
        ],
    );

    let stats = &app.practice_stats;
    let summary = format!(
        "{}: {}/{} ({:.1}%)  ·  {}: {}  ·  {}: {}  ·  {}: {}",
        app.strings().practice_solved,
        stats.solved_tasks,
        stats.total_tasks,
        stats.solved_percent(),
        app.strings().practice_started,
        stats.started_tasks,
        app.strings().practice_attempts,
        stats.total_attempts,
        app.strings().practice_first_try,
        stats.solved_first_try,
    );
    let header = Paragraph::new(summary)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .title(app.strings().practice_progress)
                .borders(Borders::ALL),
        );

    let mut items = Vec::new();
    if let Some(task) = app.resumable_practice_task() {
        items.push(ListItem::new(format!(
            "↻ {}: [{}] {}",
            app.strings().practice_continue,
            task.category,
            task.title
        )));
    }

    for category in app.practice_categories() {
        let suffix = app
            .practice_category_stats(&category)
            .map(|stats| {
                format!(
                    "  {}/{} · {} {}",
                    stats.solved_tasks,
                    stats.total_tasks,
                    app.strings().practice_attempts,
                    stats.total_attempts
                )
            })
            .unwrap_or_default();
        items.push(ListItem::new(format!("{category}{suffix}")));
    }

    let list = List::new(items)
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .block(
            Block::default()
                .title(app.strings().practice_categories)
                .borders(Borders::ALL),
        );
    let mut state = ListState::default();
    state.select(Some(app.practice_category_selected));

    let footer = Paragraph::new(app.strings().footer_practice_tasks)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(header, areas[0]);
    frame.render_stateful_widget(list, areas[1], &mut state);
    frame.render_widget(footer, areas[2]);
}
