use std::collections::{BTreeMap, BTreeSet};

use crate::{
    ContentBundle, ContentError, LabBundle, PracticeBundle, QuestionDocument, load_bundle,
    load_lab, load_practice,
};

include!(concat!(env!("OUT_DIR"), "/builtin_registry.rs"));

pub const DEFAULT_CONTENT_LOCALE: &str = "en";
pub const FALLBACK_CONTENT_LOCALE: &str = "ru";

#[derive(Debug, Clone)]
struct LocalizedBundle {
    topic_key: &'static str,
    locale: &'static str,
    bundle: ContentBundle,
}

#[derive(Debug, Clone)]
struct LocalizedPractice {
    practice_key: &'static str,
    locale: &'static str,
    bundle: PracticeBundle,
}

#[derive(Debug, Clone)]
struct LocalizedLab {
    lab_key: &'static str,
    locale: &'static str,
    bundle: LabBundle,
}

pub fn load_builtin_bundles() -> Result<Vec<ContentBundle>, ContentError> {
    load_builtin_bundles_for_locale(DEFAULT_CONTENT_LOCALE)
}

pub fn load_builtin_bundles_for_locale(
    requested_locale: &str,
) -> Result<Vec<ContentBundle>, ContentError> {
    let localized = load_all_localized_bundles()?;
    validate_localized_bundle_compatibility(&localized)?;

    let requested = normalize_locale(requested_locale);
    let base = requested.split('-').next().unwrap_or(requested.as_str());

    let mut by_topic: BTreeMap<&str, Vec<&LocalizedBundle>> = BTreeMap::new();

    for entry in &localized {
        by_topic.entry(entry.topic_key).or_default().push(entry);
    }

    let mut selected = Vec::with_capacity(by_topic.len());

    for entries in by_topic.values() {
        let chosen = entries
            .iter()
            .copied()
            .find(|entry| entry.locale == requested)
            .or_else(|| {
                if base == requested {
                    None
                } else {
                    entries.iter().copied().find(|entry| entry.locale == base)
                }
            })
            .or_else(|| {
                entries
                    .iter()
                    .copied()
                    .find(|entry| entry.locale == FALLBACK_CONTENT_LOCALE)
            })
            .or_else(|| entries.first().copied())
            .expect("localized topic group must not be empty");

        selected.push(chosen.bundle.clone());
    }

    validate_course_topic_numbers(&selected)?;

    Ok(selected)
}

pub fn load_builtin_labs_for_locale(
    requested_locale: &str,
) -> Result<Vec<LabBundle>, ContentError> {
    let localized = load_all_localized_labs()?;
    validate_localized_lab_compatibility(&localized)?;

    let requested = normalize_locale(requested_locale);
    let base = requested.split('-').next().unwrap_or(requested.as_str());

    let mut by_lab: BTreeMap<&str, Vec<&LocalizedLab>> = BTreeMap::new();

    for entry in &localized {
        by_lab.entry(entry.lab_key).or_default().push(entry);
    }

    let mut selected = Vec::with_capacity(by_lab.len());

    for entries in by_lab.values() {
        let chosen = entries
            .iter()
            .copied()
            .find(|entry| entry.locale == requested)
            .or_else(|| {
                if base == requested {
                    None
                } else {
                    entries.iter().copied().find(|entry| entry.locale == base)
                }
            })
            .or_else(|| {
                entries
                    .iter()
                    .copied()
                    .find(|entry| entry.locale == FALLBACK_CONTENT_LOCALE)
            })
            .or_else(|| entries.first().copied())
            .expect("localized lab group must not be empty");

        selected.push(chosen.bundle.clone());
    }

    validate_course_lab_numbers_and_keys(&selected)?;

    selected.sort_by_key(|bundle| {
        (
            bundle.lab.course.clone(),
            bundle.lab.sort_order,
            bundle.lab.lab_number,
        )
    });

    Ok(selected)
}

pub fn load_builtin_practice_for_locale(
    requested_locale: &str,
) -> Result<Vec<PracticeBundle>, ContentError> {
    let localized = load_all_localized_practice()?;
    validate_localized_practice_compatibility(&localized)?;

    let requested = normalize_locale(requested_locale);
    let base = requested.split('-').next().unwrap_or(requested.as_str());

    let mut by_practice: BTreeMap<&str, Vec<&LocalizedPractice>> = BTreeMap::new();

    for entry in &localized {
        by_practice
            .entry(entry.practice_key)
            .or_default()
            .push(entry);
    }

    let mut selected = Vec::with_capacity(by_practice.len());

    for entries in by_practice.values() {
        let chosen = entries
            .iter()
            .copied()
            .find(|entry| entry.locale == requested)
            .or_else(|| {
                if base == requested {
                    None
                } else {
                    entries.iter().copied().find(|entry| entry.locale == base)
                }
            })
            .or_else(|| {
                entries
                    .iter()
                    .copied()
                    .find(|entry| entry.locale == FALLBACK_CONTENT_LOCALE)
            })
            .or_else(|| entries.first().copied())
            .expect("localized practice group must not be empty");

        selected.push(chosen.bundle.clone());
    }

    selected.sort_by_key(|bundle| {
        (
            bundle.practice.category.clone(),
            bundle.practice.sort_order,
            bundle.practice.title.clone(),
        )
    });

    Ok(selected)
}

pub fn builtin_content_locales() -> Result<Vec<String>, ContentError> {
    let mut locales = load_all_localized_bundles()?
        .into_iter()
        .map(|entry| entry.locale.to_owned())
        .collect::<BTreeSet<_>>();

    locales.extend(
        load_all_localized_labs()?
            .into_iter()
            .map(|entry| entry.locale.to_owned()),
    );

    locales.extend(
        load_all_localized_practice()?
            .into_iter()
            .map(|entry| entry.locale.to_owned()),
    );

    Ok(locales.into_iter().collect())
}

fn load_all_localized_bundles() -> Result<Vec<LocalizedBundle>, ContentError> {
    BUILTIN_DOCUMENTS
        .iter()
        .map(|(topic_key, locale, topic_json, questions_json)| {
            load_bundle(topic_json, questions_json).map(|bundle| LocalizedBundle {
                topic_key,
                locale,
                bundle,
            })
        })
        .collect()
}

fn load_all_localized_practice() -> Result<Vec<LocalizedPractice>, ContentError> {
    BUILTIN_PRACTICE_DOCUMENTS
        .iter()
        .map(
            |(practice_key, locale, practice_json, setup_sql, solution_sql)| {
                load_practice(practice_json, setup_sql, solution_sql).map(|bundle| {
                    LocalizedPractice {
                        practice_key,
                        locale,
                        bundle,
                    }
                })
            },
        )
        .collect()
}

fn load_all_localized_labs() -> Result<Vec<LocalizedLab>, ContentError> {
    BUILTIN_LAB_DOCUMENTS
        .iter()
        .map(|(lab_key, locale, lab_json, lab_markdown)| {
            load_lab(lab_json, lab_markdown).map(|bundle| LocalizedLab {
                lab_key,
                locale,
                bundle,
            })
        })
        .collect()
}

fn validate_localized_practice_compatibility(
    localized: &[LocalizedPractice],
) -> Result<(), ContentError> {
    let mut by_practice: BTreeMap<&str, Vec<&LocalizedPractice>> = BTreeMap::new();

    for entry in localized {
        by_practice
            .entry(entry.practice_key)
            .or_default()
            .push(entry);
    }

    let mut errors = Vec::new();

    for (practice_key, entries) in by_practice {
        let Some(reference) = entries.first().copied() else {
            continue;
        };

        for candidate in entries.iter().copied().skip(1) {
            if reference.bundle.practice.key != candidate.bundle.practice.key
                || reference.bundle.practice.category != candidate.bundle.practice.category
                || reference.bundle.practice.practice_type
                    != candidate.bundle.practice.practice_type
                || reference.bundle.practice.comparison != candidate.bundle.practice.comparison
                || reference.bundle.practice.sort_order != candidate.bundle.practice.sort_order
                || reference.bundle.setup_sql != candidate.bundle.setup_sql
                || reference.bundle.solution_sql != candidate.bundle.solution_sql
            {
                errors.push(format!(
                    "localized practice '{practice_key}' must keep key, category, type, comparison, sort_order, setup.sql and solution.sql compatible in '{}' and '{}'",
                    reference.locale, candidate.locale
                ));
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(ContentError::Validation(errors))
    }
}

fn validate_localized_lab_compatibility(localized: &[LocalizedLab]) -> Result<(), ContentError> {
    let mut by_lab: BTreeMap<&str, Vec<&LocalizedLab>> = BTreeMap::new();

    for entry in localized {
        by_lab.entry(entry.lab_key).or_default().push(entry);
    }

    let mut errors = Vec::new();

    for (lab_key, entries) in by_lab {
        let Some(reference) = entries.first().copied() else {
            continue;
        };

        for candidate in entries.iter().copied().skip(1) {
            if reference.bundle.lab.course != candidate.bundle.lab.course
                || reference.bundle.lab.key != candidate.bundle.lab.key
                || reference.bundle.lab.lab_number != candidate.bundle.lab.lab_number
                || reference.bundle.lab.sort_order != candidate.bundle.lab.sort_order
            {
                errors.push(format!(
                    "localized lab '{lab_key}' must keep course, key, lab_number and sort_order in '{}' and '{}'",
                    reference.locale, candidate.locale
                ));
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(ContentError::Validation(errors))
    }
}

fn validate_course_lab_numbers_and_keys(labs: &[LabBundle]) -> Result<(), ContentError> {
    let mut seen_numbers = BTreeMap::<(String, i64), String>::new();
    let mut seen_keys = BTreeMap::<String, (String, i64)>::new();
    let mut errors = Vec::new();

    for bundle in labs {
        let lab = &bundle.lab;
        let number_key = (lab.course.clone(), lab.lab_number);

        if let Some(existing_key) = seen_numbers.insert(number_key.clone(), lab.key.clone()) {
            errors.push(format!(
                "course '{}' uses lab_number {} for both '{}' and '{}'",
                number_key.0, number_key.1, existing_key, lab.key
            ));
        }

        if let Some((existing_course, existing_number)) =
            seen_keys.insert(lab.key.clone(), (lab.course.clone(), lab.lab_number))
        {
            errors.push(format!(
                "lab key '{}' is reused by {} #{} and {} #{}",
                lab.key, existing_course, existing_number, lab.course, lab.lab_number
            ));
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(ContentError::Validation(errors))
    }
}

fn normalize_locale(value: &str) -> String {
    value.trim().replace('_', "-").to_ascii_lowercase()
}

fn validate_localized_bundle_compatibility(
    localized: &[LocalizedBundle],
) -> Result<(), ContentError> {
    let mut by_topic: BTreeMap<&str, Vec<&LocalizedBundle>> = BTreeMap::new();

    for entry in localized {
        by_topic.entry(entry.topic_key).or_default().push(entry);
    }

    let mut errors = Vec::new();

    for (topic_key, entries) in by_topic {
        let Some(reference) = entries.first().copied() else {
            continue;
        };

        for candidate in entries.iter().copied().skip(1) {
            compare_topic_metadata(topic_key, reference, candidate, &mut errors);
            compare_questions(topic_key, reference, candidate, &mut errors);
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(ContentError::Validation(errors))
    }
}

fn compare_topic_metadata(
    topic_key: &str,
    reference: &LocalizedBundle,
    candidate: &LocalizedBundle,
    errors: &mut Vec<String>,
) {
    if reference.bundle.topic.course != candidate.bundle.topic.course {
        errors.push(format!(
            "localized topic '{topic_key}' has different course codes in '{}' and '{}'",
            reference.locale, candidate.locale
        ));
    }

    if reference.bundle.topic.slug != candidate.bundle.topic.slug {
        errors.push(format!(
            "localized topic '{topic_key}' has different topic slugs in '{}' and '{}'",
            reference.locale, candidate.locale
        ));
    }

    if reference.bundle.topic.notes_part != candidate.bundle.topic.notes_part {
        errors.push(format!(
            "localized topic '{topic_key}' has different notes_part values in '{}' and '{}'",
            reference.locale, candidate.locale
        ));
    }

    if reference.bundle.topic.topic_number != candidate.bundle.topic.topic_number {
        errors.push(format!(
            "localized topic '{topic_key}' has different topic_number values in '{}' and '{}'",
            reference.locale, candidate.locale
        ));
    }

    if reference.bundle.topic.sort_order != candidate.bundle.topic.sort_order {
        errors.push(format!(
            "localized topic '{topic_key}' has different sort_order values in '{}' and '{}'",
            reference.locale, candidate.locale
        ));
    }
}

fn validate_course_topic_numbers(bundles: &[ContentBundle]) -> Result<(), ContentError> {
    let mut seen = BTreeMap::<(String, i64), String>::new();
    let mut errors = Vec::new();

    for bundle in bundles {
        let key = (bundle.topic.course.clone(), bundle.topic.topic_number);

        if let Some(existing_slug) = seen.insert(key.clone(), bundle.topic.slug.clone()) {
            errors.push(format!(
                "course '{}' uses topic_number {} for both '{}' and '{}'",
                key.0, key.1, existing_slug, bundle.topic.slug
            ));
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(ContentError::Validation(errors))
    }
}

fn compare_questions(
    topic_key: &str,
    reference: &LocalizedBundle,
    candidate: &LocalizedBundle,
    errors: &mut Vec<String>,
) {
    let reference_questions = questions_by_key(&reference.bundle.questions.questions);
    let candidate_questions = questions_by_key(&candidate.bundle.questions.questions);

    let reference_keys = reference_questions.keys().copied().collect::<BTreeSet<_>>();
    let candidate_keys = candidate_questions.keys().copied().collect::<BTreeSet<_>>();

    if reference_keys != candidate_keys {
        errors.push(format!(
            "localized topic '{topic_key}' must have the same question keys in '{}' and '{}'",
            reference.locale, candidate.locale
        ));
        return;
    }

    for key in reference_keys {
        let reference_question = reference_questions[key];
        let candidate_question = candidate_questions[key];

        if reference_question.question_type != candidate_question.question_type {
            errors.push(format!(
                "question '{key}' in localized topic '{topic_key}' has different question types in '{}' and '{}'",
                reference.locale, candidate.locale
            ));
        }

        if reference_question.source.key != candidate_question.source.key
            || reference_question.source.kind != candidate_question.source.kind
        {
            errors.push(format!(
                "question '{key}' in localized topic '{topic_key}' must keep the same source key and source kind in '{}' and '{}'",
                reference.locale, candidate.locale
            ));
        }

        let reference_answers = reference_question
            .answers
            .iter()
            .map(|answer| (answer.key.as_str(), answer.correct))
            .collect::<BTreeMap<_, _>>();

        let candidate_answers = candidate_question
            .answers
            .iter()
            .map(|answer| (answer.key.as_str(), answer.correct))
            .collect::<BTreeMap<_, _>>();

        if reference_answers != candidate_answers {
            errors.push(format!(
                "question '{key}' in localized topic '{topic_key}' must keep the same answer keys and correctness in '{}' and '{}'",
                reference.locale, candidate.locale
            ));
        }
    }
}

fn questions_by_key(questions: &[QuestionDocument]) -> BTreeMap<&str, &QuestionDocument> {
    questions
        .iter()
        .map(|question| (question.key.as_str(), question))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_content_is_valid() {
        let bundles = load_builtin_bundles().expect("builtin content should be valid");

        assert!(!bundles.is_empty());

        let tools_topic = bundles
            .iter()
            .find(|bundle| bundle.topic.slug == "dba1-tools-install")
            .expect("tools and installation topic should exist");

        assert!(
            tools_topic.questions.questions.len() >= 50,
            "tools and installation topic should support 50-question mode"
        );
    }

    #[test]
    fn missing_english_translation_falls_back_to_russian() {
        let bundles = load_builtin_bundles_for_locale("en")
            .expect("English locale should fall back to available content");

        let tools_topic = bundles
            .iter()
            .find(|bundle| bundle.topic.slug == "dba1-tools-install")
            .expect("tools and installation topic should exist through fallback");

        assert!(!tools_topic.topic.title.trim().is_empty());
    }

    #[test]
    fn locale_normalization_accepts_region_and_underscore() {
        assert_eq!(normalize_locale("EN_us"), "en-us");
    }

    #[test]
    fn builtin_practice_is_valid_and_localized() {
        let practice = load_builtin_practice_for_locale("ru")
            .expect("builtin practice content should be valid");

        let task = practice
            .iter()
            .find(|bundle| bundle.practice.key == "sql-clients-without-orders")
            .expect("demo practice task should exist");

        assert_eq!(task.practice.category, "joins");
        assert!(task.practice.title.contains("Клиенты"));
        assert!(!task.setup_sql.trim().is_empty());
        assert!(!task.solution_sql.trim().is_empty());
    }

    #[test]
    fn builtin_labs_are_valid_and_fall_back_to_russian() {
        let labs = load_builtin_labs_for_locale("en")
            .expect("English locale should fall back to available lab content");

        assert!(!labs.is_empty());
        assert!(labs.iter().any(|bundle| bundle.lab.lab_number == 23));
        assert!(labs.iter().all(|bundle| !bundle.body.trim().is_empty()));
    }
}
