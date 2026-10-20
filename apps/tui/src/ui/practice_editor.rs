use ratatui::{
    Frame,
    layout::{Alignment, Constraint},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use dba_trainer_domain::PracticeExecutionResult;

use crate::app::App;

use super::common::{render_message, vertical_chunks};

pub fn render(frame: &mut Frame, app: &App) {
    let Some(task) = app.current_practice_task.as_ref() else {
        render_message(
            frame,
            app.strings().practice_title,
            app.strings().practice_missing,
            app.strings().footer_back,
        );
        return;
    };

    let areas = vertical_chunks(
        frame.area(),
        &[
            Constraint::Length(5),
            Constraint::Percentage(45),
            Constraint::Min(7),
            Constraint::Length(3),
        ],
    );

    let task_text = format!(
        "{}\n{}: {}",
        task.description,
        app.strings().practice_category,
        task.category
    );
    let task_panel = Paragraph::new(task_text).wrap(Wrap { trim: true }).block(
        Block::default()
            .title(task.title.as_str())
            .borders(Borders::ALL),
    );

    let mut editor_text = app.practice_sql.clone();
    let cursor = app.practice_cursor.min(editor_text.len());
    editor_text.insert(cursor, '▌');
    let editor = Paragraph::new(editor_text)
        .wrap(Wrap { trim: false })
        .block(
            Block::default()
                .title(app.strings().sql_editor)
                .borders(Borders::ALL),
        );

    let output_text = render_output(app);
    let output = Paragraph::new(output_text)
        .wrap(Wrap { trim: false })
        .block(
            Block::default()
                .title(app.strings().practice_output)
                .borders(Borders::ALL),
        );

    let footer = Paragraph::new(app.strings().footer_practice_editor)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(task_panel, areas[0]);
    frame.render_widget(editor, areas[1]);
    frame.render_widget(output, areas[2]);
    frame.render_widget(footer, areas[3]);
}

fn render_output(app: &App) -> String {
    let mut lines = Vec::new();

    if let Some(check) = app.practice_check.as_ref() {
        let status = if check.is_correct {
            app.strings().practice_correct
        } else {
            app.strings().practice_incorrect
        };

        lines.push(format!(
            "{}: {status} ({}: {}, {}: {})",
            app.strings().practice_check,
            app.strings().practice_actual_rows,
            check.actual_rows,
            app.strings().practice_expected_rows,
            check.expected_rows,
        ));
        lines.push(String::new());
    }

    match app.practice_output.as_ref() {
        Some(result) => lines.extend(execution_lines(app, result)),
        None if lines.is_empty() => lines.push(app.strings().practice_no_output.to_string()),
        None => {}
    }

    lines.join("\n")
}

fn execution_lines(app: &App, result: &PracticeExecutionResult) -> Vec<String> {
    if let Some(affected_rows) = result.affected_rows {
        return vec![format!(
            "{}: {affected_rows}",
            app.strings().practice_affected_rows
        )];
    }

    let mut lines = Vec::new();

    if !result.columns.is_empty() {
        lines.push(result.columns.join(" | "));
        lines.push(
            result
                .columns
                .iter()
                .map(|column| "-".repeat(column.chars().count().max(3)))
                .collect::<Vec<_>>()
                .join("-+-"),
        );
    }

    for row in &result.rows {
        lines.push(
            row.iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(" | "),
        );
    }

    lines.push(format!(
        "{}: {}",
        app.strings().practice_actual_rows,
        result.row_count()
    ));

    lines
}
