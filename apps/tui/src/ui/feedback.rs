use dba_trainer_domain::{AnswerOptionId, Question, Source, SourceKind};

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, Wrap},
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
        .constraints([Constraint::Min(3), Constraint::Length(3)])
        .split(frame.area());

    let selected = format_options(question, &result.selected_option_ids);

    let correct = format_options(question, &result.correct_option_ids);

    let answer_word = if result.correct_option_ids.len() > 1 {
        "Правильные ответы"
    } else {
        "Правильный ответ"
    };

    let status = if result.is_correct {
        "Верно"
    } else {
        "Неверно"
    };

    let source = format_source(&question.source);

    let text = format!(
        "\
Ваш ответ:
{selected}

{answer_word}:
{correct}

Объяснение:
{}

Источник:
{}",
        question.explanation, source,
    );

    let body = Paragraph::new(text)
        .wrap(Wrap { trim: false })
        .scroll((app.feedback_scroll, 0))
        .block(
            Block::default()
                .title(format!("Результат: {status}"))
                .borders(Borders::ALL),
        );

    let footer =
        Paragraph::new("↑/↓ — прокрутка | Enter — следующий вопрос | Esc — отмена | q — выйти")
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));

    frame.render_widget(body, areas[0]);

    frame.render_widget(footer, areas[1]);
}

fn format_options(question: &Question, option_ids: &[AnswerOptionId]) -> String {
    let options = question
        .options
        .iter()
        .filter(|option| option_ids.contains(&option.id))
        .map(|option| format!("• {}", option.text,))
        .collect::<Vec<_>>();

    if options.is_empty() {
        String::from("• вариант не найден")
    } else {
        options.join("\n")
    }
}

fn format_source(source: &Source) -> String {
    match source.kind {
        SourceKind::CourseMaterial => {
            format!(
                "Материал курса\n{} / {}\n{}",
                source.module, source.section, source.locator,
            )
        }

        SourceKind::PostgreSqlDocs => match source.url.as_deref() {
            Some(url) => {
                format!(
                    "Документация PostgreSQL\n{} / {}\n{}\n{}",
                    source.module, source.section, source.locator, url,
                )
            }

            None => {
                format!(
                    "Документация PostgreSQL\n{} / {}\n{}",
                    source.module, source.section, source.locator,
                )
            }
        },
    }
}
