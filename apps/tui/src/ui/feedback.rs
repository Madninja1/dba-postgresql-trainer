use dba_trainer_domain::{AnswerOptionId, Question, Source, SourceKind};

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::{app::App, localization::UiStrings};

use super::common::render_message;

pub fn render(frame: &mut Frame, app: &App) {
    let strings = app.strings();

    let (Some(question), Some(result)) = (app.current_question.as_ref(), app.feedback.as_ref())
    else {
        render_message(
            frame,
            strings.answer,
            strings.answer_missing,
            strings.footer_feedback,
        );

        return;
    };

    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(3)])
        .split(frame.area());

    let selected = format_options(strings, question, &result.selected_option_ids);
    let correct = format_options(strings, question, &result.correct_option_ids);

    let answer_word = if result.correct_option_ids.len() > 1 {
        strings.correct_answers
    } else {
        strings.correct_answer
    };

    let status = if result.is_correct {
        strings.correct
    } else {
        strings.incorrect
    };

    let source = format_source(strings, &question.source);

    let text = format!(
        "{}:\n{}\n\n{}:\n{}\n\n{}:\n{}\n\n{}:\n{}",
        strings.your_answer,
        selected,
        answer_word,
        correct,
        strings.explanation,
        question.explanation,
        strings.source,
        source,
    );

    let body = Paragraph::new(text)
        .wrap(Wrap { trim: false })
        .scroll((app.feedback_scroll, 0))
        .block(
            Block::default()
                .title(format!("{}: {status}", strings.result))
                .borders(Borders::ALL),
        );

    let footer = Paragraph::new(strings.footer_feedback)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(body, areas[0]);
    frame.render_widget(footer, areas[1]);
}

fn format_options(
    strings: &UiStrings,
    question: &Question,
    option_ids: &[AnswerOptionId],
) -> String {
    let options = question
        .options
        .iter()
        .filter(|option| option_ids.contains(&option.id))
        .map(|option| format!("• {}", option.text))
        .collect::<Vec<_>>();

    if options.is_empty() {
        strings.option_missing.to_string()
    } else {
        options.join("\n")
    }
}

fn format_source(strings: &UiStrings, source: &Source) -> String {
    match source.kind {
        SourceKind::CourseMaterial => {
            format!(
                "{}\n{} / {}\n{}",
                strings.course_material, source.module, source.section, source.locator,
            )
        }

        SourceKind::PostgreSqlDocs => match source.url.as_deref() {
            Some(url) => {
                format!(
                    "{}\n{} / {}\n{}\n{}",
                    strings.postgresql_docs, source.module, source.section, source.locator, url,
                )
            }

            None => {
                format!(
                    "{}\n{} / {}\n{}",
                    strings.postgresql_docs, source.module, source.section, source.locator,
                )
            }
        },
    }
}
