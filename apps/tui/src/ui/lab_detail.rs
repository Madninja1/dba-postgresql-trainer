use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::app::App;

use super::common::render_message;

pub fn render(frame: &mut Frame, app: &App) {
    let Some(lab) = app.selected_lab.as_ref() else {
        render_message(
            frame,
            app.strings().labs_title,
            app.strings().labs_empty,
            app.strings().footer_back,
        );
        return;
    };

    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(3)])
        .split(frame.area());

    let title = format!("{}. {}", lab.lab_number, lab.title);

    let body = Paragraph::new(lab.body.as_str())
        .wrap(Wrap { trim: false })
        .scroll((app.lab_scroll, 0))
        .block(Block::default().title(title).borders(Borders::ALL));

    let footer = Paragraph::new(app.strings().footer_lab_detail)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(body, areas[0]);
    frame.render_widget(footer, areas[1]);
}
