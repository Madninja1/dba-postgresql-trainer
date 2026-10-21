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
    let xp_opportunity = app.quiz_xp_opportunity_for_scope(&app.quiz_scope);
    let show_xp_details = xp_opportunity.is_some_and(|opportunity| {
        opportunity.unique_question_xp > 0
            || opportunity.mastery_xp > 0
            || opportunity.course_completion_xp > 0
    });
    let constraints = if show_xp_details {
        vec![
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(3),
        ]
    } else {
        vec![
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(5),
            Constraint::Length(3),
        ]
    };
    let areas = vertical_chunks(frame.area(), &constraints);

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
            let label = if let Some(opportunity) = xp_opportunity {
                let bonus = opportunity.pass_bonus_for_limit(*limit);
                if bonus > 0 {
                    format!(
                        "{} · +{} XP {}",
                        app.strings().question_limit(*limit),
                        bonus,
                        app.strings().xp_pass_bonus,
                    )
                } else {
                    format!(
                        "{} · {}",
                        app.strings().question_limit(*limit),
                        app.strings().xp_received,
                    )
                }
            } else {
                app.strings().question_limit(*limit).to_string()
            };

            ListItem::new(label).style(style)
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

    if show_xp_details {
        let opportunity = xp_opportunity.expect("XP details require an opportunity");
        let mut lines = Vec::new();
        if opportunity.unique_question_xp > 0 {
            lines.push(format!(
                "{}: +{} XP",
                app.strings().xp_questions,
                opportunity.unique_question_xp,
            ));
        }
        if opportunity.mastery_xp > 0 {
            lines.push(format!(
                "{}: +{} XP",
                app.strings().xp_topics,
                opportunity.mastery_xp,
            ));
        }
        if opportunity.course_completion_xp > 0 {
            lines.push(format!(
                "{}: +{} XP",
                app.strings().xp_courses,
                opportunity.course_completion_xp,
            ));
        }
        let xp = Paragraph::new(lines.join("\n")).block(
            Block::default()
                .title(app.strings().xp_available)
                .borders(Borders::ALL),
        );
        frame.render_widget(xp, areas[3]);
        frame.render_widget(footer, areas[4]);
    } else {
        frame.render_widget(footer, areas[3]);
    }
}
