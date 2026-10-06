use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};

use crate::app::App;

use super::common::render_message;

pub fn render(frame: &mut Frame, app: &App) {
    let (Some(question), Some(result)) = (app.current_question.as_ref(), app.feedback.as_ref())
    else {
        render_message(
            frame,
            "Ответ",
            "Результат ответа отсутствует.",
            "Enter — продолжить",
        );

        return;
    };

    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(6),
            Constraint::Length(10),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title = if result.is_correct {
        "Верно"
    } else {
        "Неверно"
    };

    let header = Paragraph::new(title)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    let items = question
        .options
        .iter()
        .map(|option| {
            let correct = result.correct_option_ids.contains(&option.id);

            let selected = result.selected_option_ids.contains(&option.id);

            let marker = match (correct, selected) {
                (true, true) => "✓",

                (true, false) => "✓",

                (false, true) => "✗",

                (false, false) => " ",
            };

            ListItem::new(format!("{marker} {}", option.text))
        })
        .collect::<Vec<_>>();

    let answers = List::new(items).block(
        Block::default()
            .title("Правильный ответ")
            .borders(Borders::ALL),
    );

    let source = match question.source.url.as_deref() {
        Some(url) => {
            format!(
                "{} / {}\n{}\n{}",
                question.source.module, question.source.section, question.source.locator, url,
            )
        }

        None => {
            format!(
                "{} / {}\n{}",
                question.source.module, question.source.section, question.source.locator,
            )
        }
    };

    let explanation = Paragraph::new(format!("{}\n\nИсточник:\n{}", question.explanation, source,))
        .wrap(Wrap { trim: true })
        .block(Block::default().title("Объяснение").borders(Borders::ALL));

    let footer = Paragraph::new("Enter — следующий вопрос | Esc — отмена теста | q — выйти")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(header, areas[0]);

    frame.render_widget(answers, areas[1]);

    frame.render_widget(explanation, areas[2]);

    frame.render_widget(footer, areas[3]);
}
