use dba_trainer_domain::QuestionType;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};

use crate::app::App;

use super::common::render_message;

pub fn render(frame: &mut Frame, app: &App) {
    let Some(question) = app.current_question.as_ref() else {
        render_message(frame, "Тест", "Вопрос не загружен.", "q — выход");

        return;
    };

    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(6),
            Constraint::Min(6),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let header = Paragraph::new(format!(
        "Вопрос {}/{}",
        app.answered_questions + 1,
        app.total_questions,
    ))
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL));

    let question_text = Paragraph::new(question.text.as_str())
        .wrap(Wrap { trim: true })
        .block(Block::default().title("Вопрос").borders(Borders::ALL));

    let items = question
        .options
        .iter()
        .enumerate()
        .map(|(index, option)| {
            let cursor = if index == app.option_selected {
                ">"
            } else {
                " "
            };

            let marker = match question.question_type {
                QuestionType::SingleChoice => "",

                QuestionType::MultipleChoice => {
                    if app.selected_answer_ids.contains(&option.id) {
                        "[x]"
                    } else {
                        "[ ]"
                    }
                }
            };

            let style = if index == app.option_selected {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };

            ListItem::new(format!("{cursor} {marker} {}", option.text)).style(style)
        })
        .collect::<Vec<_>>();

    let answers = List::new(items).block(Block::default().title("Ответы").borders(Borders::ALL));

    let footer_text = match question.question_type {
        QuestionType::SingleChoice => "↑/↓ или j/k — выбор | Enter — ответить | q — выход",

        QuestionType::MultipleChoice => {
            "↑/↓ или j/k — выбор | Space — отметить | Enter — ответить | q — выход"
        }
    };

    let footer = Paragraph::new(footer_text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(header, areas[0]);

    frame.render_widget(question_text, areas[1]);

    frame.render_widget(answers, areas[2]);

    frame.render_widget(footer, areas[3]);
}
