use dba_trainer_domain::{StatisticsLimit, TrainingStats};

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::{App, StatisticsView, statistics_limit_label};

pub fn render(frame: &mut Frame, app: &App) {
    match &app.statistics_view {
        StatisticsView::Root => render_menu(
            frame,
            "Статистика",
            &[
                String::from("Общая"),
                String::from("По курсам и темам"),
                String::from("По режимам"),
            ],
            app.statistics_selected,
        ),

        StatisticsView::Courses => {
            let labels = app
                .statistics_course_codes()
                .into_iter()
                .map(|course_code| {
                    let accuracy = accuracy_label(app.statistics_for_course(&course_code));

                    format!("{}    {}", course_code.to_uppercase(), accuracy,)
                })
                .collect::<Vec<_>>();

            render_menu(
                frame,
                "Статистика → Курсы",
                &labels,
                app.statistics_selected,
            );
        }

        StatisticsView::Course(course_code) => {
            let course_accuracy = accuracy_label(app.statistics_for_course(course_code));
            let mut labels = vec![format!("Все темы    {course_accuracy}")];

            labels.extend(
                app.statistics_topics_for_course(course_code)
                    .into_iter()
                    .enumerate()
                    .map(|(index, topic)| {
                        let accuracy = accuracy_label(app.statistics_for_topic(topic.id));

                        format!("{:02}. {}    {}", index + 1, topic.title, accuracy,)
                    }),
            );

            render_menu(
                frame,
                &format!("Статистика → {}", course_code.to_uppercase()),
                &labels,
                app.statistics_selected,
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

                    format!("{}    {}", statistics_limit_label(limit), accuracy,)
                })
                .collect::<Vec<_>>();

            render_menu(
                frame,
                &format!("Статистика → {title}"),
                &labels,
                app.statistics_selected,
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

                    format!("{}    {}", statistics_limit_label(limit), accuracy,)
                })
                .collect::<Vec<_>>();

            render_menu(
                frame,
                "Статистика → Режимы",
                &labels,
                app.statistics_selected,
            );
        }

        StatisticsView::Detail { filter, title } => {
            let title = if title.is_empty() {
                statistics_limit_label(filter.limit).to_string()
            } else {
                title.clone()
            };

            render_detail(frame, app, &title);
        }
    }
}

fn render_menu(frame: &mut Frame, title: &str, labels: &[String], selected: usize) {
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

    let footer = Paragraph::new(
        "↑/↓ — выбор | Enter — открыть | Esc/Backspace — назад | c — очистить всю статистику | q — выход",
    )
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL));

    frame.render_widget(list, areas[0]);
    frame.render_widget(footer, areas[1]);
}

fn render_detail(frame: &mut Frame, app: &App, title: &str) {
    let Some(stats) = app.statistics.as_ref() else {
        let paragraph = Paragraph::new("Статистика не загружена.")
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
        "Завершено тестов: {}\n\
Отменено тестов: {}\n\n\
Отвечено вопросов: {}\n\
Правильных ответов: {}\n\
Неправильных ответов: {}\n\n\
Точность: {:.1}%",
        stats.completed_sessions,
        stats.cancelled_sessions,
        stats.answered_questions,
        stats.correct_answers,
        stats.incorrect_answers(),
        stats.accuracy_percent(),
    );

    let body = Paragraph::new(message)
        .alignment(Alignment::Center)
        .block(Block::default().title(title).borders(Borders::ALL));

    let footer = Paragraph::new(
        "Enter, Esc или Backspace — назад | c — очистить всю статистику | q — выход",
    )
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
