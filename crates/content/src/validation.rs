use std::collections::{HashMap, HashSet};

use crate::model::{
    CONTENT_SCHEMA_VERSION, LabDocument, QuestionTypeDocument, QuestionsDocument, SourceDocument,
    SourceKindDocument, TopicDocument,
};

pub(crate) fn validate_bundle(
    topic: &TopicDocument,
    questions: &QuestionsDocument,
) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    validate_topic(topic, &mut errors);

    validate_questions(topic, questions, &mut errors);

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_topic(topic: &TopicDocument, errors: &mut Vec<String>) {
    if topic.schema_version != CONTENT_SCHEMA_VERSION {
        errors.push(format!(
            "unsupported topic schema version: {}",
            topic.schema_version
        ));
    }

    if !valid_course_code(&topic.course) {
        errors.push(format!("invalid course code: {}", topic.course));
    }

    if !valid_key(&topic.slug) {
        errors.push(format!("invalid topic slug: {}", topic.slug));
    }

    if topic.notes_part <= 0 {
        errors.push(String::from("topic notes_part must be greater than zero"));
    }

    if topic.topic_number <= 0 {
        errors.push(String::from("topic topic_number must be greater than zero"));
    }

    if topic.title.trim().is_empty() {
        errors.push(String::from("topic title must not be empty"));
    }

    if let Some(description) = &topic.description
        && description.trim().is_empty()
    {
        errors.push(String::from(
            "topic description must not be empty when present",
        ));
    }

    if topic.sort_order < 0 {
        errors.push(String::from("topic sort_order must not be negative"));
    }
}

fn validate_questions(
    topic: &TopicDocument,
    document: &QuestionsDocument,
    errors: &mut Vec<String>,
) {
    if document.schema_version != CONTENT_SCHEMA_VERSION {
        errors.push(format!(
            "unsupported questions schema version: {}",
            document.schema_version
        ));
    }

    if document.topic != topic.slug {
        errors.push(format!(
            "questions topic '{}' does not match topic slug '{}'",
            document.topic, topic.slug
        ));
    }

    if document.questions.is_empty() {
        errors.push(String::from("topic must contain at least one question"));
    }

    let mut question_keys = HashSet::new();

    let mut sources_by_key: HashMap<&str, &SourceDocument> = HashMap::new();

    for question in &document.questions {
        if !valid_key(&question.key) {
            errors.push(format!("invalid question key: {}", question.key));
        }

        if !question_keys.insert(question.key.as_str()) {
            errors.push(format!("duplicate question key: {}", question.key));
        }

        if question.text.trim().is_empty() {
            errors.push(format!("question '{}' has empty text", question.key));
        }

        if question.explanation.trim().is_empty() {
            errors.push(format!("question '{}' has empty explanation", question.key));
        }

        validate_answers(
            question.key.as_str(),
            question.question_type,
            &question.answers,
            errors,
        );

        validate_source(question.key.as_str(), &question.source, errors);

        if let Some(existing) = sources_by_key.get(question.source.key.as_str()) {
            if *existing != &question.source {
                errors.push(format!(
                    "source key '{}' is used with different source data",
                    question.source.key
                ));
            }
        } else {
            sources_by_key.insert(question.source.key.as_str(), &question.source);
        }
    }
}

fn validate_answers(
    question_key: &str,
    question_type: QuestionTypeDocument,
    answers: &[crate::model::AnswerDocument],
    errors: &mut Vec<String>,
) {
    if answers.len() < 2 {
        errors.push(format!(
            "question '{question_key}' must have at least two answers"
        ));
    }

    let mut answer_keys = HashSet::new();

    let mut answer_texts = HashSet::new();

    for answer in answers {
        if !valid_key(&answer.key) {
            errors.push(format!(
                "question '{question_key}' has invalid answer key '{}'",
                answer.key
            ));
        }

        if !answer_keys.insert(answer.key.as_str()) {
            errors.push(format!(
                "question '{question_key}' has duplicate answer key '{}'",
                answer.key
            ));
        }

        let text = answer.text.trim();

        if text.is_empty() {
            errors.push(format!(
                "question '{question_key}' has an answer with empty text"
            ));
        }

        if !answer_texts.insert(text) {
            errors.push(format!(
                "question '{question_key}' has duplicate answer text '{text}'"
            ));
        }
    }

    let correct_count = answers.iter().filter(|answer| answer.correct).count();

    let incorrect_count = answers.len() - correct_count;

    match question_type {
        QuestionTypeDocument::SingleChoice => {
            if correct_count != 1 {
                errors.push(format!(
                    "single-choice question '{question_key}' must have exactly one correct answer"
                ));
            }
        }

        QuestionTypeDocument::MultipleChoice => {
            if correct_count < 2 {
                errors.push(
                        format!(
                            "multiple-choice question '{question_key}' must have at least two correct answers"
                        ),
                    );
            }

            if incorrect_count == 0 {
                errors.push(
                        format!(
                            "multiple-choice question '{question_key}' must have at least one incorrect answer"
                        ),
                    );
            }
        }
    }
}

fn validate_source(question_key: &str, source: &SourceDocument, errors: &mut Vec<String>) {
    if !valid_key(&source.key) {
        errors.push(format!(
            "question '{question_key}' has invalid source key '{}'",
            source.key
        ));
    }

    if source.module.trim().is_empty() {
        errors.push(format!(
            "question '{question_key}' has source with empty module"
        ));
    }

    if source.section.trim().is_empty() {
        errors.push(format!(
            "question '{question_key}' has source with empty section"
        ));
    }

    if source.locator.trim().is_empty() {
        errors.push(format!(
            "question '{question_key}' has source with empty locator"
        ));
    }

    if source.kind == SourceKindDocument::PostgreSqlDocs {
        match source.url.as_deref() {
            Some(url) if valid_url(url) => {
                // valid
            }

            _ => {
                errors.push(
                    format!(
                        "PostgreSQL documentation source for question '{question_key}' must have an http or https URL"
                    ),
                );
            }
        }
    }
}

fn valid_course_code(value: &str) -> bool {
    let Some((track, number)) = value.rsplit_once('-') else {
        return false;
    };

    if track.is_empty() || number.is_empty() {
        return false;
    }

    if !track
        .chars()
        .next()
        .is_some_and(|character| character.is_ascii_lowercase())
    {
        return false;
    }

    if track.split('-').any(str::is_empty)
        || !track.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        })
    {
        return false;
    }

    if !number.chars().all(|character| character.is_ascii_digit()) {
        return false;
    }

    number.parse::<u32>().is_ok_and(|number| number > 0)
}

fn valid_key(value: &str) -> bool {
    if value.is_empty() {
        return false;
    }

    if value.starts_with('-') || value.ends_with('-') {
        return false;
    }

    value.chars().all(|character| {
        character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
    })
}

fn valid_url(value: &str) -> bool {
    let value = value.trim();

    value.starts_with("https://") || value.starts_with("http://")
}

pub fn validate_lab(lab: &LabDocument, body: &str) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    if lab.schema_version != CONTENT_SCHEMA_VERSION {
        errors.push(format!(
            "unsupported lab schema_version {}, expected {}",
            lab.schema_version, CONTENT_SCHEMA_VERSION
        ));
    }

    if !valid_course_code(&lab.course) {
        errors.push(format!("invalid lab course code '{}'", lab.course));
    }

    if !valid_key(&lab.key) {
        errors.push(format!("invalid lab key '{}'", lab.key));
    }

    if lab.lab_number <= 0 {
        errors.push(format!("lab '{}' must have lab_number > 0", lab.key));
    }

    if lab.title.trim().is_empty() {
        errors.push(format!("lab '{}' must have a non-empty title", lab.key));
    }

    if let Some(description) = &lab.description
        && description.trim().is_empty()
    {
        errors.push(format!(
            "lab '{}' description must not be empty when present",
            lab.key
        ));
    }

    if lab.sort_order < 0 {
        errors.push(format!("lab '{}' sort_order must not be negative", lab.key));
    }

    if body.trim().is_empty() {
        errors.push(format!("lab '{}' must have a non-empty body", lab.key));
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub fn validate_practice(
    practice: &crate::model::PracticeDocument,
    setup_sql: &str,
    solution_sql: &str,
) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    if practice.schema_version != CONTENT_SCHEMA_VERSION {
        errors.push(format!(
            "unsupported practice schema_version {}, expected {}",
            practice.schema_version, CONTENT_SCHEMA_VERSION
        ));
    }

    if !valid_key(&practice.key) {
        errors.push(format!("invalid practice key '{}'", practice.key));
    }

    if !valid_key(&practice.category) {
        errors.push(format!("invalid practice category '{}'", practice.category));
    }

    if practice.title.trim().is_empty() {
        errors.push(format!(
            "practice '{}' must have a non-empty title",
            practice.key
        ));
    }

    if practice.description.trim().is_empty() {
        errors.push(format!(
            "practice '{}' must have a non-empty description",
            practice.key
        ));
    }

    if practice.sort_order < 0 {
        errors.push(format!(
            "practice '{}' sort_order must not be negative",
            practice.key
        ));
    }

    if setup_sql.trim().is_empty() {
        errors.push(format!(
            "practice '{}' must have non-empty setup.sql",
            practice.key
        ));
    }

    if solution_sql.trim().is_empty() {
        errors.push(format!(
            "practice '{}' must have non-empty solution.sql",
            practice.key
        ));
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::valid_course_code;

    #[test]
    fn accepts_supported_course_code_shape() {
        for value in [
            "dba-1",
            "dba-01",
            "rust-1",
            "sql-1",
            "python-1",
            "postgresql-dba-2",
            "rust2026-1",
        ] {
            assert!(valid_course_code(value), "expected '{value}' to be valid");
        }
    }

    #[test]
    fn rejects_invalid_course_codes() {
        for value in [
            "",
            "dba",
            "DBA-1",
            "rust-0",
            "rust-x",
            "-1",
            "1-1",
            "rust--advanced-1",
            "rust_advanced-1",
        ] {
            assert!(
                !valid_course_code(value),
                "expected '{value}' to be invalid"
            );
        }
    }
}
