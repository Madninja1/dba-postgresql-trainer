use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};

use crate::app::App;

pub fn render(frame: &mut Frame, app: &App) {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),
            Constraint::Min(4),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let warning = Paragraph::new(
        "Отменённый тест нельзя будет продолжить.\n\
             Его ответы сохранятся в базе, но не будут учитываться \
             в основной статистике.",
    )
    .alignment(Alignment::Center)
    .wrap(Wrap { trim: true })
    .block(Block::default().title("Отмена теста").borders(Borders::ALL));

    let labels = ["Продолжить тест", "Отменить тест"];

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

    let actions = List::new(items).block(Block::default().title("Действие").borders(Borders::ALL));

    let footer = Paragraph::new("↑/↓ — выбор | Enter — подтвердить | Esc — назад | q — выйти")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(warning, areas[0]);

    frame.render_widget(actions, areas[1]);

    frame.render_widget(footer, areas[2]);
}
