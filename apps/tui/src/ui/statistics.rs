use ratatui::Frame;

use super::common::render_message;

pub fn render(frame: &mut Frame) {
    render_message(
        frame,
        "Статистика",
        "Статистика пока недоступна",
        "Esc - назад | q - выход",
    )
}
