use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
};

use dba_trainer_domain::{PracticeExecutionResult, PracticeRule, PracticeValue};

use crate::app::{App, PracticePane};

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

    let compact = frame.area().height < 36;
    let areas = if compact {
        vertical_chunks(
            frame.area(),
            &[
                Constraint::Length(7),
                Constraint::Min(10),
                Constraint::Length(3),
            ],
        )
    } else {
        vertical_chunks(
            frame.area(),
            &[
                Constraint::Length(7),
                Constraint::Min(12),
                Constraint::Length(9),
                Constraint::Length(3),
            ],
        )
    };

    let progress = app.progress_for_practice_task(task.id);
    let status = match progress {
        Some(progress) if progress.solved => app.strings().practice_solved,
        Some(progress) if progress.is_started() => app.strings().practice_started,
        _ => "—",
    };
    let attempts = progress.map_or(0, |progress| progress.attempts);
    let task_text = format!(
        "{}\n{}: {}  ·  {}: {}  ·  {}: {}  ·  {}: {}",
        task.description,
        app.strings().practice_category,
        task.category,
        app.strings().practice_difficulty,
        app.strings().practice_difficulty(task.difficulty),
        app.strings().practice_attempts,
        attempts,
        app.strings().practice_progress,
        status,
    );
    let task_panel = Paragraph::new(task_text).wrap(Wrap { trim: true }).block(
        Block::default()
            .title(task.title.as_str())
            .borders(Borders::ALL),
    );

    frame.render_widget(task_panel, areas[0]);
    render_workbench(frame, app, areas[1], compact);

    let footer_index = if compact {
        2
    } else {
        render_learning_panel(
            frame,
            app,
            areas[2],
            app.practice_pane == PracticePane::Learning,
        );
        3
    };

    let footer = Paragraph::new(app.strings().footer_practice_editor)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(footer, areas[footer_index]);
}

fn render_workbench(frame: &mut Frame, app: &App, area: Rect, compact: bool) {
    let horizontal = area.width >= 100;
    let panes = Layout::default()
        .direction(if horizontal {
            Direction::Horizontal
        } else {
            Direction::Vertical
        })
        .constraints([Constraint::Percentage(58), Constraint::Percentage(42)])
        .split(area);

    render_editor(frame, app, panes[0]);
    render_inspector(frame, app, panes[1], compact);
}

fn render_editor(frame: &mut Frame, app: &App, area: Rect) {
    let mut editor_text = app.practice_sql.clone();
    let cursor = app.practice_cursor.min(editor_text.len());
    let cursor_line = editor_text[..cursor]
        .chars()
        .filter(|character| *character == '\n')
        .count();
    editor_text.insert(cursor, '|');
    let visible_height = area.height.saturating_sub(2) as usize;
    let scroll = cursor_line
        .saturating_sub(visible_height.saturating_sub(1))
        .min(u16::MAX as usize) as u16;
    let active = app.practice_pane == PracticePane::Editor;
    let title = focus_title(app.strings().sql_editor, active);
    let block = Block::default().title(title).borders(Borders::ALL);
    let editor = Paragraph::new(editor_text)
        .wrap(Wrap { trim: false })
        .scroll((scroll, 0))
        .block(block);
    frame.render_widget(editor, area);
}

fn render_inspector(frame: &mut Frame, app: &App, area: Rect, compact: bool) {
    match app.practice_pane {
        PracticePane::Editor | PracticePane::Schema => render_schema(frame, app, area),
        PracticePane::Data => render_data(frame, app, area),
        PracticePane::Result => render_result(frame, app, area),
        PracticePane::Learning if compact => render_learning_panel(frame, app, area, true),
        PracticePane::Learning => render_schema(frame, app, area),
    }
}

fn render_schema(frame: &mut Frame, app: &App, area: Rect) {
    let active = app.practice_pane == PracticePane::Schema;
    let title = focus_title(app.strings().practice_schema, active);
    let mut lines = Vec::new();

    for (index, table) in app.practice_schema.tables.iter().enumerate() {
        let marker = if index == app.practice_schema_selected {
            ">"
        } else {
            " "
        };
        lines.push(format!("{marker} {}", table.name));
        if index == app.practice_schema_selected {
            for column in &table.columns {
                let mut flags = Vec::new();
                if column.primary_key {
                    flags.push("PK");
                }
                if column.not_null {
                    flags.push("NOT NULL");
                }
                let flags = if flags.is_empty() {
                    String::new()
                } else {
                    format!(" [{}]", flags.join(", "))
                };
                let default = column
                    .default_value
                    .as_ref()
                    .map(|value| format!(" DEFAULT {value}"))
                    .unwrap_or_default();
                lines.push(format!(
                    "    {}  {}{}{}",
                    column.name, column.data_type, flags, default
                ));
            }
        }
    }

    if lines.is_empty() {
        lines.push(String::from("(empty schema)"));
    } else {
        lines.push(String::new());
        lines.push(app.strings().practice_enter_table.to_string());
    }

    let body = Paragraph::new(lines.join("\n"))
        .wrap(Wrap { trim: false })
        .block(Block::default().title(title).borders(Borders::ALL));
    frame.render_widget(body, area);
}

fn render_data(frame: &mut Frame, app: &App, area: Rect) {
    let title = app
        .practice_table_data
        .as_ref()
        .map(|data| {
            format!(
                "{} — {} | Esc -> {}",
                app.strings().practice_data,
                data.table,
                app.strings().practice_schema
            )
        })
        .unwrap_or_else(|| app.strings().practice_data.to_string());
    let lines = app
        .practice_table_data
        .as_ref()
        .map(|data| matrix_lines(&data.result, app.strings().practice_affected_rows))
        .unwrap_or_else(|| vec![String::from("(select a table in Schema and press Enter)")]);
    render_scrolled_text(
        frame,
        app,
        area,
        &title,
        lines,
        app.practice_pane == PracticePane::Data,
    );
}

fn render_result(frame: &mut Frame, app: &App, area: Rect) {
    let mut lines = Vec::new();

    if let Some(check) = app.practice_check.as_ref() {
        let status = if check.is_correct {
            app.strings().practice_correct
        } else {
            app.strings().practice_incorrect
        };
        lines.push(format!("{}: {status}", app.strings().practice_check));
        lines.push(format!(
            "{}: {}  ·  {}: {}",
            app.strings().practice_actual_rows,
            check.actual_rows,
            app.strings().practice_expected_rows,
            check.expected_rows
        ));
        if !check.failed_checks.is_empty() {
            lines.push(format!("{}:", app.strings().practice_failed_checks));
            lines.extend(
                check
                    .failed_checks
                    .iter()
                    .map(|check| format!("  • {check}")),
            );
        }
        lines.push(String::new());
    }

    if let Some(result) = app.practice_output.as_ref() {
        lines.extend(matrix_lines(result, app.strings().practice_affected_rows));
    } else if lines.is_empty() {
        lines.push(app.strings().practice_no_output.to_string());
    }

    render_scrolled_text(
        frame,
        app,
        area,
        app.strings().practice_output,
        lines,
        app.practice_pane == PracticePane::Result,
    );
}

fn render_scrolled_text(
    frame: &mut Frame,
    app: &App,
    area: Rect,
    title: &str,
    lines: Vec<String>,
    active: bool,
) {
    let visible_height = area.height.saturating_sub(2) as usize;
    let start = app.practice_scroll.min(lines.len().saturating_sub(1));
    let width = area.width.saturating_sub(2) as usize;
    let visible = lines
        .into_iter()
        .skip(start)
        .take(visible_height)
        .map(|line| horizontal_slice(&line, app.practice_horizontal_scroll, width))
        .map(Line::from)
        .collect::<Vec<_>>();
    let body = Paragraph::new(visible).block(
        Block::default()
            .title(focus_title(title, active))
            .borders(Borders::ALL),
    );
    frame.render_widget(body, area);
}

fn render_learning_panel(frame: &mut Frame, app: &App, area: Rect, active: bool) {
    let Some(task) = app.current_practice_task.as_ref() else {
        return;
    };
    let mut lines = Vec::new();

    if !task.rules.is_empty() {
        let rules = task
            .rules
            .iter()
            .map(rule_text)
            .collect::<Vec<_>>()
            .join("; ");
        lines.push(format!("Rules: {rules}"));
    }

    if app.practice_hint_count > 0 {
        lines.push(format!("{}:", app.strings().practice_hints));
        for (index, hint) in task.hints.iter().take(app.practice_hint_count).enumerate() {
            lines.push(format!("  {}. {hint}", index + 1));
        }
    }

    if app.practice_show_solution {
        lines.push(format!("{}:", app.strings().practice_solution));
        lines.extend(task.solution_sql.lines().map(|line| format!("  {line}")));
        if let Some(explanation) = task.solution_explanation.as_deref() {
            lines.push(format!("  {explanation}"));
        }
    } else if !app.practice_solution_available() {
        lines.push(app.strings().practice_solution_locked.to_string());
    }

    if let Some(message) = app.practice_message.as_deref() {
        lines.push(message.to_string());
    }

    if let Some(error) = app.practice_error.as_deref() {
        lines.push(String::new());
        lines.push(error.to_string());
    }

    if lines.is_empty() {
        lines.push(String::from("F3 — hint · F4 — solution"));
    }

    render_scrolled_text(
        frame,
        app,
        area,
        app.strings().practice_learning,
        lines,
        active,
    );
}

fn matrix_lines(result: &PracticeExecutionResult, affected_rows_label: &str) -> Vec<String> {
    if let Some(affected_rows) = result.affected_rows {
        return vec![format!("{affected_rows_label}: {affected_rows}")];
    }

    if result.columns.is_empty() {
        return vec![String::from("(no columns)")];
    }

    let widths = result
        .columns
        .iter()
        .enumerate()
        .map(|(column, name)| {
            result
                .rows
                .iter()
                .filter_map(|row| row.get(column))
                .map(value_width)
                .fold(name.chars().count(), usize::max)
                .clamp(3, 30)
        })
        .collect::<Vec<_>>();

    let mut lines = Vec::new();
    lines.push(format_row(
        result.columns.iter().map(String::as_str),
        &widths,
    ));
    lines.push(
        widths
            .iter()
            .map(|width| "-".repeat(*width))
            .collect::<Vec<_>>()
            .join("-+-"),
    );
    for row in &result.rows {
        let values = row.iter().map(ToString::to_string).collect::<Vec<_>>();
        lines.push(format_row(values.iter().map(String::as_str), &widths));
    }
    lines.push(format!("Rows: {}", result.row_count()));
    lines
}

fn format_row<'a>(values: impl Iterator<Item = &'a str>, widths: &[usize]) -> String {
    values
        .zip(widths.iter().copied())
        .map(|(value, width)| {
            let shortened = truncate(value, width);
            format!("{shortened:<width$}")
        })
        .collect::<Vec<_>>()
        .join(" | ")
}

fn value_width(value: &PracticeValue) -> usize {
    value.to_string().chars().count()
}

fn truncate(value: &str, width: usize) -> String {
    if value.chars().count() <= width {
        return value.to_string();
    }
    if width <= 1 {
        return "…".to_string();
    }
    let mut result = value.chars().take(width - 1).collect::<String>();
    result.push('…');
    result
}

fn horizontal_slice(value: &str, start: usize, width: usize) -> String {
    value.chars().skip(start).take(width).collect()
}

fn rule_text(rule: &PracticeRule) -> String {
    match rule {
        PracticeRule::RequireKeyword(keyword) => format!("require {keyword}"),
        PracticeRule::ForbidKeyword(keyword) => format!("forbid {keyword}"),
        PracticeRule::ForbidSelectStar => String::from("forbid SELECT *"),
    }
}

fn focus_title(title: &str, active: bool) -> String {
    if active {
        format!("> {title} <")
    } else {
        title.to_string()
    }
}
