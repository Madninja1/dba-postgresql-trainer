use dba_trainer_domain::{ActivityStats, Source, SourceKind, StatisticsLimit, TrainingStats};

use ratatui::{
    Frame,
    layout::{Alignment, Constraint},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
};

use crate::app::{App, StatisticsView};

use super::common::{content_area, topic_display_number, vertical_chunks};

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
                strings.mistakes.to_string(),
                strings.practice.to_string(),
                strings.activity.to_string(),
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

            let topics = app.statistics_topics_for_course(course_code);

            labels.extend(topics.iter().enumerate().map(|(index, topic)| {
                let accuracy = accuracy_label(app.statistics_for_topic(topic.id));
                let display_number = topic_display_number(&topics, index);

                format!("{display_number}. {}    {}", topic.title, accuracy)
            }));

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

        StatisticsView::Mistakes => render_mistakes(frame, app),

        StatisticsView::MistakeCourse(course_code) => {
            render_mistake_course(frame, app, course_code)
        }

        StatisticsView::MistakeTopic {
            course_code,
            topic_id,
        } => render_mistake_topic(frame, app, course_code, *topic_id),

        StatisticsView::MistakeDetail => render_mistake_detail(frame, app),

        StatisticsView::Practice => render_practice_stats(frame, app),

        StatisticsView::Activity => render_activity(frame, app),

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
    let areas = vertical_chunks(frame.area(), &[Constraint::Min(5), Constraint::Length(3)]);

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
    let mut state = ListState::default();

    if !labels.is_empty() {
        state.select(Some(selected.min(labels.len().saturating_sub(1))));
    }

    let footer = Paragraph::new(footer_text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_stateful_widget(list, areas[0], &mut state);
    frame.render_widget(footer, areas[1]);
}

fn render_mistakes(frame: &mut Frame, app: &App) {
    let strings = app.strings();

    if app.mistake_summaries.is_empty() {
        render_message_with_footer(
            frame,
            strings.mistakes,
            strings.no_mistakes,
            strings.footer_statistics,
            0,
        );
        return;
    }

    let course_codes = app.mistake_course_codes();
    let mut labels = Vec::with_capacity(course_codes.len() + 1);
    labels.push(format!(
        "{} ({})",
        strings.work_on_mistakes,
        app.mistake_summaries.len()
    ));

    labels.extend(course_codes.iter().map(|course_code| {
        format!(
            "{} ({})",
            course_code.to_uppercase(),
            app.mistake_count_for_course(course_code),
        )
    }));

    render_menu(
        frame,
        strings.mistakes,
        &labels,
        app.statistics_selected,
        strings.footer_statistics,
    );
}

fn render_mistake_course(frame: &mut Frame, app: &App, course_code: &str) {
    let strings = app.strings();
    let topics = app.mistake_topics_for_course(course_code);
    let mut labels = Vec::with_capacity(topics.len() + 1);

    labels.push(format!(
        "{} ({})",
        strings.review_course_mistakes,
        app.mistake_count_for_course(course_code),
    ));

    labels.extend(topics.iter().enumerate().map(|(index, topic)| {
        let display_number = topic_display_number(&topics, index);
        format!(
            "{display_number}. {} ({})",
            topic.title,
            app.mistake_count_for_topic(topic.id),
        )
    }));

    render_menu(
        frame,
        &format!("{} → {}", strings.mistakes, course_code.to_uppercase()),
        &labels,
        app.statistics_selected,
        strings.footer_statistics,
    );
}

fn render_mistake_topic(
    frame: &mut Frame,
    app: &App,
    course_code: &str,
    topic_id: dba_trainer_domain::TopicId,
) {
    let strings = app.strings();
    let summaries = app.mistakes_for_topic(topic_id);
    let topic_title = app
        .topic_by_id(topic_id)
        .map(|topic| topic.title.as_str())
        .unwrap_or(strings.topic);

    let mut labels = Vec::with_capacity(summaries.len() + 1);
    labels.push(format!(
        "{} ({})",
        strings.review_topic_mistakes,
        summaries.len()
    ));

    labels.extend(summaries.iter().map(|summary| {
        format!(
            "{} / {} · {:.1}% — {}",
            summary.incorrect_attempts,
            summary.total_attempts,
            summary.error_rate_percent(),
            summary.question_text.replace('\n', " "),
        )
    }));

    render_menu(
        frame,
        &format!(
            "{} → {} → {}",
            strings.mistakes,
            course_code.to_uppercase(),
            topic_title,
        ),
        &labels,
        app.statistics_selected,
        strings.footer_statistics,
    );
}

fn render_mistake_detail(frame: &mut Frame, app: &App) {
    let strings = app.strings();
    let Some(detail) = app.mistake_detail.as_ref() else {
        render_message_with_footer(
            frame,
            strings.mistakes,
            strings.statistics_unavailable,
            strings.footer_statistics_scroll,
            app.statistics_scroll,
        );
        return;
    };

    let mut message = format!(
        "[{}] {}\n\n{}\n\n{}: {} / {} ({:.1}%)\n{}: {}\n\n{}:\n{}\n\n{}:\n{}",
        detail.summary.course_code.to_uppercase(),
        detail.summary.topic_title,
        detail.question.text,
        strings.incorrect_answers_count,
        detail.summary.incorrect_attempts,
        detail.summary.total_attempts,
        detail.summary.error_rate_percent(),
        strings.last_error,
        detail.summary.last_incorrect_at,
        strings.correct_answers,
        detail.correct_answers.join("\n"),
        strings.explanation,
        detail.question.explanation,
    );

    message.push_str(&format!(
        "\n\n{}:\n{}",
        strings.source,
        format_source(strings, &detail.question.source),
    ));

    message.push_str(&format!("\n\n{}:\n", strings.mistake_history));

    for (index, attempt) in detail.attempts.iter().enumerate() {
        let result = if attempt.is_correct {
            strings.correct
        } else {
            strings.incorrect
        };
        let selected = if attempt.selected_answers.is_empty() {
            "—".to_string()
        } else {
            attempt.selected_answers.join(" | ")
        };

        message.push_str(&format!(
            "\n{}. [{}] {}\n{}: {}\n",
            index + 1,
            result,
            attempt.answered_at,
            strings.your_answer,
            selected,
        ));
    }

    render_message_with_footer(
        frame,
        strings.mistakes,
        &message,
        strings.footer_statistics_scroll,
        app.statistics_scroll,
    );
}

fn render_practice_stats(frame: &mut Frame, app: &App) {
    let strings = app.strings();
    let stats = &app.practice_stats;
    let mut lines = vec![
        format!(
            "{}: {}/{} ({:.1}%)",
            strings.practice_solved,
            stats.solved_tasks,
            stats.total_tasks,
            stats.solved_percent()
        ),
        format!("{}: {}", strings.practice_started, stats.started_tasks),
        format!("{}: {}", strings.practice_attempts, stats.total_attempts),
        format!("{}: {}", strings.practice_first_try, stats.solved_first_try),
        String::new(),
    ];

    for category in &stats.categories {
        lines.push(format!(
            "{}: {}/{} · {} {} · {} {}",
            category.category,
            category.solved_tasks,
            category.total_tasks,
            strings.practice_started,
            category.started_tasks,
            strings.practice_attempts,
            category.total_attempts,
        ));
    }

    render_message_with_footer(
        frame,
        strings.practice_title,
        &lines.join("\n"),
        strings.footer_statistics_scroll,
        app.statistics_scroll,
    );
}

fn render_activity(frame: &mut Frame, app: &App) {
    let strings = app.strings();
    let Some(stats) = app.activity_stats.as_ref() else {
        render_message_with_footer(
            frame,
            strings.activity,
            strings.statistics_unavailable,
            strings.footer_statistics_scroll,
            app.statistics_scroll,
        );
        return;
    };

    let level = stats.level();
    let today_accuracy = if stats.today_answered_questions == 0 {
        0.0
    } else {
        stats.today_correct_answers as f64 / stats.today_answered_questions as f64 * 100.0
    };

    let mut message = format!(
        "{}: {}\n{}\n\n{}: {} · {} / {} · {:.1}%\n{}: {}\n{}: {}\n{}: {}\n{}: {}\n{}: {}\n{}: {}\n\n{}: {:.1}%\n{}\n\n{}:\n",
        strings.rank,
        strings.progress_level(level),
        strings.progress_message(level),
        strings.today,
        stats.today_answered_questions,
        stats.today_correct_answers,
        stats.today_answered_questions,
        today_accuracy,
        strings.active_days,
        stats.active_days,
        strings.study_days,
        stats.study_days,
        strings.answered_questions,
        stats.total_answered_questions,
        strings.current_streak,
        stats.current_streak,
        strings.best_streak,
        stats.best_streak,
        strings.last_7_days,
        stats.last_7_days_answered_questions,
        strings.career_path,
        stats.overall_mastery_percent(),
        strings.salary_disclaimer,
        strings.course_progress,
    );

    if stats.course_progress.is_empty() {
        message.push_str("—\n");
    } else {
        for progress in &stats.course_progress {
            message.push_str(&format!(
                "{}: {}/{} ({:.1}%)\n",
                progress.course_code.to_uppercase(),
                progress.passed_topics,
                progress.total_topics,
                progress.completion_percent(),
            ));
        }
    }

    message.push_str(&format!(
        "\n{}\n{}:\n{}\n",
        strings.study_day_target,
        strings.recent_activity,
        heatmap(stats),
    ));

    for day in stats.recent_days.iter().rev() {
        message.push_str(&format!(
            "{} — {}: {} · {}: {}\n",
            day.date,
            strings.answered_questions,
            day.answered_questions,
            strings.correct_answers_count,
            day.correct_answers,
        ));
    }

    render_message_with_footer(
        frame,
        strings.activity,
        &message,
        strings.footer_statistics_scroll,
        app.statistics_scroll,
    );
}

fn heatmap(stats: &ActivityStats) -> String {
    if stats.recent_days.is_empty() {
        return String::from("—");
    }

    let first = stats
        .recent_days
        .first()
        .map(|day| day.date.as_str())
        .unwrap_or("");
    let last = stats
        .recent_days
        .last()
        .map(|day| day.date.as_str())
        .unwrap_or("");
    let mut rows = Vec::new();

    for week in stats.recent_days.chunks(7) {
        let row = week
            .iter()
            .map(|day| match day.answered_questions {
                0 => '·',
                1..=9 => '░',
                10..=24 => '▒',
                25..=49 => '▓',
                _ => '█',
            })
            .collect::<String>();
        rows.push(row);
    }

    format!("{first} → {last}\n{}", rows.join("\n"))
}

fn render_message_with_footer(
    frame: &mut Frame,
    title: &str,
    message: &str,
    footer_text: &str,
    scroll: u16,
) {
    let areas = vertical_chunks(frame.area(), &[Constraint::Min(5), Constraint::Length(3)]);

    let body = Paragraph::new(message)
        .wrap(Wrap { trim: false })
        .scroll((scroll, 0))
        .block(Block::default().title(title).borders(Borders::ALL));

    let footer = Paragraph::new(footer_text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(body, areas[0]);
    frame.render_widget(footer, areas[1]);
}

fn render_detail(frame: &mut Frame, app: &App, title: &str) {
    let strings = app.strings();

    let Some(stats) = app.statistics.as_ref() else {
        let paragraph = Paragraph::new(strings.statistics_unavailable)
            .alignment(Alignment::Center)
            .block(Block::default().title(title).borders(Borders::ALL));

        frame.render_widget(paragraph, content_area(frame.area()));
        return;
    };

    let areas = vertical_chunks(frame.area(), &[Constraint::Min(8), Constraint::Length(3)]);

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

fn format_source(strings: &crate::localization::UiStrings, source: &Source) -> String {
    match source.kind {
        SourceKind::CourseMaterial => format!(
            "{}\n{} / {}\n{}",
            strings.course_material, source.module, source.section, source.locator,
        ),
        SourceKind::PostgreSqlDocs => match source.url.as_deref() {
            Some(url) => format!(
                "{}\n{} / {}\n{}\n{}",
                strings.postgresql_docs, source.module, source.section, source.locator, url,
            ),
            None => format!(
                "{}\n{} / {}\n{}",
                strings.postgresql_docs, source.module, source.section, source.locator,
            ),
        },
    }
}
