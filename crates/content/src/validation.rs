use std::collections::{HashMap, HashSet};

use crate::model::{
    CONTENT_SCHEMA_VERSION, QuestionTypeDocument, QuestionsDocument, SourceDocument,
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
    let Some(number) = value.strip_prefix("dba-") else {
        return false;
    };

    if number.is_empty() {
        return false;
    }

    if !number.chars().all(|character| character.is_ascii_digit()) {
        return false;
    }

    match number.parse::<u32>() {
        Ok(number) => number > 0,

        Err(_) => false,
    }
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
