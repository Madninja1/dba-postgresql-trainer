use ratatui::Frame;

use super::common::render_message;

pub fn render(frame: &mut Frame) {
    render_message(
        frame,
        "Темы DBA",
        "Темы пока не загружены",
        "Esc - назад | q - выход",
    );
}
