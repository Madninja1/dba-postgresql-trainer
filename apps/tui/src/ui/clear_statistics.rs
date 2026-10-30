use ratatui::{
    Frame,
    layout::{Alignment, Constraint},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};

use crate::app::App;

use super::common::vertical_chunks;

pub fn render(frame: &mut Frame, app: &App) {
    let strings = app.strings();
    let warning_text = strings
        .clear_statistics_warning
        .replace("{scope}", &app.statistics_clear_label);

    let areas = vertical_chunks(
        frame.area(),
        &[
            Constraint::Length(6),
            Constraint::Min(4),
            Constraint::Length(3),
        ],
    );

    let warning = Paragraph::new(warning_text)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .title(strings.clear_statistics_title)
                .borders(Borders::ALL),
        );

    let labels = [
        strings.clear_statistics_confirm,
        strings.clear_statistics_cancel,
    ];
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

    let footer = Paragraph::new(strings.footer_cancel)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(warning, areas[0]);
    frame.render_widget(actions, areas[1]);
    frame.render_widget(footer, areas[2]);
}
