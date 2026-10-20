use ratatui::{
    Frame,
    layout::{Alignment, Constraint},
    style::{Modifier, Style},
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::app::App;

use super::common::{render_message, vertical_chunks};

pub fn render(frame: &mut Frame, app: &App) {
    let Some(lab) = app.selected_lab.as_ref() else {
        render_message(
            frame,
            app.strings().labs_title,
            app.strings().labs_empty,
            app.strings().footer_back,
        );
        return;
    };

    let areas = vertical_chunks(frame.area(), &[Constraint::Min(5), Constraint::Length(3)]);

    let title = format!("{}. {}", lab.lab_number, lab.title);

    let body = Paragraph::new(markdown_to_text(&lab.body))
        .wrap(Wrap { trim: false })
        .scroll((app.lab_scroll, 0))
        .block(Block::default().title(title).borders(Borders::ALL));

    let footer = Paragraph::new(app.strings().footer_lab_detail)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(body, areas[0]);
    frame.render_widget(footer, areas[1]);
}

fn markdown_to_text(markdown: &str) -> Text<'static> {
    let mut lines = Vec::new();
    let mut in_code = false;
    let mut first_content_seen = false;

    for raw_line in markdown.lines() {
        let line = raw_line.trim_end();

        if let Some(language) = line.strip_prefix("```") {
            if in_code {
                lines.push(Line::styled(
                    "└────────────────────────────────────────",
                    Style::default().add_modifier(Modifier::DIM),
                ));
                in_code = false;
            } else {
                let label = language.trim();
                let fence = if label.is_empty() {
                    "┌─ code".to_string()
                } else {
                    format!("┌─ {label}")
                };

                lines.push(Line::styled(
                    fence,
                    Style::default().add_modifier(Modifier::BOLD),
                ));
                in_code = true;
            }

            first_content_seen = true;
            continue;
        }

        if in_code {
            lines.push(Line::from(format!("│ {line}")));
            continue;
        }

        if line.trim().is_empty() {
            lines.push(Line::default());
            continue;
        }

        if !first_content_seen && line.starts_with("# ") {
            // The lab title is already displayed in the surrounding TUI block.
            first_content_seen = true;
            continue;
        }

        first_content_seen = true;

        if let Some((level, heading)) = markdown_heading(line) {
            let modifier = if level <= 2 {
                Modifier::BOLD | Modifier::UNDERLINED
            } else {
                Modifier::BOLD
            };

            lines.push(Line::styled(
                clean_inline_markdown(heading),
                Style::default().add_modifier(modifier),
            ));
            continue;
        }

        if let Some(quote) = line.strip_prefix("> ") {
            lines.push(Line::styled(
                format!("│ {}", clean_inline_markdown(quote)),
                Style::default().add_modifier(Modifier::ITALIC),
            ));
            continue;
        }

        if let Some(item) = line.strip_prefix("- ").or_else(|| line.strip_prefix("* ")) {
            lines.push(Line::from(format!("  • {}", clean_inline_markdown(item))));
            continue;
        }

        if let Some((number, item)) = numbered_list_item(line) {
            lines.push(Line::from(format!(
                "  {number}. {}",
                clean_inline_markdown(item)
            )));
            continue;
        }

        if is_markdown_table_separator(line) {
            lines.push(Line::styled(
                "  ────────────────────────────────────────",
                Style::default().add_modifier(Modifier::DIM),
            ));
            continue;
        }

        if let Some(cells) = markdown_table_cells(line) {
            lines.push(Line::from(format!(
                "  {}",
                cells
                    .into_iter()
                    .map(clean_inline_markdown)
                    .collect::<Vec<_>>()
                    .join("  │  ")
            )));
            continue;
        }

        if line == "---" || line == "***" || line == "___" {
            lines.push(Line::styled(
                "──────────────────────────────────────────",
                Style::default().add_modifier(Modifier::DIM),
            ));
            continue;
        }

        lines.push(Line::from(clean_inline_markdown(line)));
    }

    if in_code {
        lines.push(Line::styled(
            "└────────────────────────────────────────",
            Style::default().add_modifier(Modifier::DIM),
        ));
    }

    Text::from(lines)
}

fn markdown_heading(line: &str) -> Option<(usize, &str)> {
    let level = line
        .chars()
        .take_while(|character| *character == '#')
        .count();

    if level == 0 || level > 6 {
        return None;
    }

    line.get(level..)
        .and_then(|rest| rest.strip_prefix(' '))
        .map(|heading| (level, heading))
}

fn numbered_list_item(line: &str) -> Option<(&str, &str)> {
    let (number, rest) = line.split_once(". ")?;

    if number.is_empty() || !number.chars().all(|character| character.is_ascii_digit()) {
        return None;
    }

    Some((number, rest))
}

fn is_markdown_table_separator(line: &str) -> bool {
    if !line.trim_start().starts_with('|') {
        return false;
    }

    let cells = line.trim().trim_matches('|').split('|').map(str::trim);

    let mut saw_cell = false;
    for cell in cells {
        saw_cell = true;
        let cell = cell.trim_matches(':');
        if cell.len() < 3 || !cell.chars().all(|character| character == '-') {
            return false;
        }
    }

    saw_cell
}

fn markdown_table_cells(line: &str) -> Option<Vec<&str>> {
    if !line.trim_start().starts_with('|') {
        return None;
    }

    let cells = line
        .trim()
        .trim_matches('|')
        .split('|')
        .map(str::trim)
        .collect::<Vec<_>>();

    (!cells.is_empty()).then_some(cells)
}

fn clean_inline_markdown(line: &str) -> String {
    line.replace("**", "").replace("__", "").replace('`', "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markdown_helpers_remove_common_markup_tokens() {
        assert_eq!(
            clean_inline_markdown("**Important**: run `psql`"),
            "Important: run psql"
        );
        assert_eq!(markdown_heading("## Step"), Some((2, "Step")));
        assert_eq!(markdown_heading("plain text"), None);
        assert_eq!(numbered_list_item("12. Question"), Some(("12", "Question")));
        assert!(is_markdown_table_separator("|---|---:|"));
        assert_eq!(markdown_table_cells("| A | B |"), Some(vec!["A", "B"]));
    }
}
