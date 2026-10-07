use dba_trainer_domain::{StatisticsLimit, TrainingStats};

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::{App, StatisticsView};

pub fn render(frame: &mut Frame, app: &App) {
    let strings = app.strings();

    match &app.statistics_view {
        StatisticsView::Root => render_menu(
            frame,
            strings.statistics,
            &[
                strings.overall.to_string(),
                strings.by_courses_topics.to_string(),
                strings.by_modes.to_string(),
            ],
            app.statistics_selected,
            strings.footer_statistics,
        ),

        StatisticsView::Courses => {
            let labels = app
                .statistics_course_codes()
                .into_iter()
                .map(|course_code| {
                    let accuracy = accuracy_label(app.statistics_for_course(&course_code));
                    format!("{}    {}", course_code.to_uppercase(), accuracy)
                })
                .collect::<Vec<_>>();

            render_menu(
                frame,
                strings.statistics_courses,
                &labels,
                app.statistics_selected,
                strings.footer_statistics,
            );
        }

        StatisticsView::Course(course_code) => {
            let course_accuracy = accuracy_label(app.statistics_for_course(course_code));
            let mut labels = vec![format!("{}    {course_accuracy}", strings.all_topics)];

            labels.extend(
                app.statistics_topics_for_course(course_code)
                    .into_iter()
                    .enumerate()
                    .map(|(index, topic)| {
                        let accuracy = accuracy_label(app.statistics_for_topic(topic.id));
                        format!("{:02}. {}    {}", index + 1, topic.title, accuracy)
                    }),
            );

            render_menu(
                frame,
                &format!("{} → {}", strings.statistics, course_code.to_uppercase()),
                &labels,
                app.statistics_selected,
                strings.footer_statistics,
            );
        }

        StatisticsView::Filters { title, .. } => {
            let limits = [
                StatisticsLimit::Any,
                StatisticsLimit::Twenty,
                StatisticsLimit::Fifty,
                StatisticsLimit::AllQuestions,
            ];

            let labels = limits
                .into_iter()
                .map(|limit| {
                    let accuracy = accuracy_label(app.statistics_for_filter_limit(limit));
                    format!("{}    {}", strings.statistics_limit(limit), accuracy)
                })
                .collect::<Vec<_>>();

            render_menu(
                frame,
                &format!("{} → {title}", strings.statistics),
                &labels,
                app.statistics_selected,
                strings.footer_statistics,
            );
        }

        StatisticsView::Modes => {
            let limits = [
                StatisticsLimit::Twenty,
                StatisticsLimit::Fifty,
                StatisticsLimit::AllQuestions,
            ];

            let labels = limits
                .into_iter()
                .map(|limit| {
                    let accuracy = accuracy_label(app.statistics_for_mode(limit));
                    format!("{}    {}", strings.statistics_limit(limit), accuracy)
                })
                .collect::<Vec<_>>();

            render_menu(
                frame,
                strings.statistics_modes,
                &labels,
                app.statistics_selected,
                strings.footer_statistics,
            );
        }

        StatisticsView::Detail { filter, title } => {
            let title = if title.is_empty() {
                strings.statistics_limit(filter.limit).to_string()
            } else {
                title.clone()
            };

            render_detail(frame, app, &title);
        }
    }
}

fn render_menu(
    frame: &mut Frame,
    title: &str,
    labels: &[String],
    selected: usize,
    footer_text: &str,
) {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(3)])
        .split(frame.area());

    let items = labels
        .iter()
        .enumerate()
        .map(|(index, label)| {
            let style = if index == selected {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };

            ListItem::new(label.as_str()).style(style)
        })
        .collect::<Vec<_>>();

    let list = List::new(items).block(Block::default().title(title).borders(Borders::ALL));

    let footer = Paragraph::new(footer_text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(list, areas[0]);
    frame.render_widget(footer, areas[1]);
}

fn render_detail(frame: &mut Frame, app: &App, title: &str) {
    let strings = app.strings();

    let Some(stats) = app.statistics.as_ref() else {
        let paragraph = Paragraph::new(strings.statistics_unavailable)
            .alignment(Alignment::Center)
            .block(Block::default().title(title).borders(Borders::ALL));

        frame.render_widget(paragraph, frame.area());
        return;
    };

    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(8), Constraint::Length(3)])
        .split(frame.area());

    let message = format!(
        "{}: {}\n{}: {}\n\n{}: {}\n{}: {}\n{}: {}\n\n{}: {:.1}%",
        strings.completed_tests,
        stats.completed_sessions,
        strings.cancelled_tests,
        stats.cancelled_sessions,
        strings.answered_questions,
        stats.answered_questions,
        strings.correct_answers_count,
        stats.correct_answers,
        strings.incorrect_answers_count,
        stats.incorrect_answers(),
        strings.accuracy,
        stats.accuracy_percent(),
    );

    let body = Paragraph::new(message)
        .alignment(Alignment::Center)
        .block(Block::default().title(title).borders(Borders::ALL));

    let footer = Paragraph::new(strings.footer_statistics_detail)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(body, areas[0]);
    frame.render_widget(footer, areas[1]);
}

fn accuracy_label(stats: Option<&TrainingStats>) -> String {
    match stats {
        Some(stats) if stats.answered_questions > 0 => {
            format!("{:.1}%", stats.accuracy_percent())
        }
        _ => String::from("—"),
    }
}
