use dba_trainer_domain::{QuestionLimit, QuizScope};

use crate::app::{App, QUESTION_LIMITS};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use super::common::vertical_chunks;

pub fn render(frame: &mut Frame, app: &App) {
    let areas = vertical_chunks(
        frame.area(),
        &[
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(5),
            Constraint::Length(3),
        ],
    );

    let title = match &app.quiz_scope {
        QuizScope::AllTopics => app.strings().general_quiz.to_string(),

        QuizScope::Course(course_code) => {
            format!(
                "{}: {}",
                app.strings().general_quiz,
                course_code.to_uppercase()
            )
        }

        QuizScope::Topic(topic_id) => app
            .topics
            .iter()
            .find(|topic| topic.id == *topic_id)
            .map(|topic| format!("{}: {}", app.strings().topic, topic.title))
            .unwrap_or_else(|| app.strings().topic_quiz.to_string()),

        QuizScope::Mistakes => app.strings().work_on_mistakes.to_string(),

        QuizScope::MistakesCourse(course_code) => format!(
            "{}: {}",
            app.strings().review_course_mistakes,
            course_code.to_uppercase()
        ),

        QuizScope::MistakesTopic(topic_id) => app
            .topic_by_id(*topic_id)
            .map(|topic| format!("{}: {}", app.strings().review_topic_mistakes, topic.title))
            .unwrap_or_else(|| app.strings().review_topic_mistakes.to_string()),
    };

    let header = Paragraph::new(title)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    let items = QUESTION_LIMITS
        .iter()
        .enumerate()
        .map(|(index, limit)| {
            let style = if index == app.limit_selected {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };

            ListItem::new(app.strings().question_limit(*limit)).style(style)
        })
        .collect::<Vec<_>>();

    let list = List::new(items).block(Block::default().borders(Borders::ALL));

    let criteria = Paragraph::new(format!(
        "{} — {:.0}% ({}/20)\n{} — {:.0}% ({}/50)\n{} — {:.0}%",
        app.strings().question_limit(QuestionLimit::Twenty),
        QuestionLimit::Twenty.pass_threshold_percent(),
        QuestionLimit::Twenty.required_correct_answers(20),
        app.strings().question_limit(QuestionLimit::Fifty),
        QuestionLimit::Fifty.pass_threshold_percent(),
        QuestionLimit::Fifty.required_correct_answers(50),
        app.strings().question_limit(QuestionLimit::All),
        QuestionLimit::All.pass_threshold_percent(),
    ))
    .block(
        Block::default()
            .title(app.strings().pass_criteria)
            .borders(Borders::ALL),
    );

    let footer = Paragraph::new(app.strings().footer_setup)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(header, areas[0]);
    frame.render_widget(list, areas[1]);
    frame.render_widget(criteria, areas[2]);
    frame.render_widget(footer, areas[3]);
}
