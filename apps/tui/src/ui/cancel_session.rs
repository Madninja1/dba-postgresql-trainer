use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};

use crate::app::App;

pub fn render(frame: &mut Frame, app: &App) {
    let strings = app.strings();

    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),
            Constraint::Min(4),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let warning = Paragraph::new(strings.cancel_warning)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .title(strings.cancel_title)
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

    let footer = Paragraph::new(strings.footer_cancel)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(warning, areas[0]);
    frame.render_widget(actions, areas[1]);
    frame.render_widget(footer, areas[2]);
}
