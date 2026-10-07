use std::{fs, path::Path};

use crate::{
    error::ContentError,
    model::{ContentBundle, QuestionsDocument, TopicDocument},
    validation::validate_bundle,
};

pub fn load_bundle(topic_json: &str, questions_json: &str) -> Result<ContentBundle, ContentError> {
    let topic: TopicDocument =
        serde_json::from_str(topic_json).map_err(|source| ContentError::Json {
            document: "topic.json",
            source,
        })?;

    let questions: QuestionsDocument =
        serde_json::from_str(questions_json).map_err(|source| ContentError::Json {
            document: "questions.json",
            source,
        })?;

    validate_bundle(&topic, &questions).map_err(ContentError::Validation)?;

    Ok(ContentBundle { topic, questions })
}

pub fn load_bundle_from_dir(directory: impl AsRef<Path>) -> Result<ContentBundle, ContentError> {
    let directory = directory.as_ref();

    let topic_path = directory.join("topic.json");

    let questions_path = directory.join("questions.json");

    let topic_json = fs::read_to_string(&topic_path).map_err(|source| ContentError::Io {
        path: topic_path.clone(),
        source,
    })?;

    let questions_json =
        fs::read_to_string(&questions_path).map_err(|source| ContentError::Io {
            path: questions_path.clone(),
            source,
        })?;

    load_bundle(&topic_json, &questions_json)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{ContentError, QuestionTypeDocument};

    const TOPIC_JSON: &str = r#"
    {
        "schema_version": 1,
        "course": "dba-1",
        "slug": "dba1-tools-install",
        "notes_part": 1,
        "topic_number": 1,
        "title": "Инструменты и установка",
        "description": "Test topic",
        "sort_order": 10
    }
    "#;

    #[test]
    fn loads_valid_bundle() {
        let questions_json = r#"
            {
                "schema_version": 1,
                "topic": "dba1-tools-install",
                "questions": [
                    {
                        "key": "dba1-tools-001",
                        "type": "single_choice",
                        "text": "Which command?",
                        "answers": [
                            {
                                "key": "a",
                                "text": "A",
                                "correct": false
                            },
                            {
                                "key": "b",
                                "text": "B",
                                "correct": true
                            }
                        ],
                        "explanation": "Explanation",
                        "source": {
                            "key": "source-1",
                            "kind": "course_material",
                            "module": "DBA-1",
                            "section": "Test",
                            "locator": "slide 1",
                            "url": null
                        }
                    },
                    {
                        "key": "dba1-tools-002",
                        "type": "multiple_choice",
                        "text": "Select all correct answers",
                        "answers": [
                            {
                                "key": "a",
                                "text": "A",
                                "correct": true
                            },
                            {
                                "key": "b",
                                "text": "B",
                                "correct": true
                            },
                            {
                                "key": "c",
                                "text": "C",
                                "correct": false
                            }
                        ],
                        "explanation": "Explanation",
                        "source": {
                            "key": "source-2",
                            "kind": "postgresql_docs",
                            "module": "PostgreSQL 16",
                            "section": "Test",
                            "locator": "test section",
                            "url": "https://postgrespro.ru/docs/postgresql/16/"
                        }
                    }
                ]
            }
            "#;

        let bundle = load_bundle(TOPIC_JSON, questions_json).expect("valid content should load");

        assert_eq!(bundle.topic.course, "dba-1");

        assert_eq!(bundle.topic.slug, "dba1-tools-install");

        assert_eq!(bundle.questions.questions.len(), 2);

        assert_eq!(
            bundle.questions.questions[0].question_type,
            QuestionTypeDocument::SingleChoice
        );

        assert_eq!(
            bundle.questions.questions[1].question_type,
            QuestionTypeDocument::MultipleChoice
        );
    }

    #[test]
    fn accepts_other_dba_course_numbers() {
        let topic_json = r#"
            {
                "schema_version": 1,
                "course": "dba-3",
                "slug": "dba3-test-topic",
                "notes_part": 1,
                "topic_number": 1,
                "title": "DBA-3 Test",
                "description": null,
                "sort_order": 10
            }
            "#;

        let questions_json = r#"
            {
                "schema_version": 1,
                "topic": "dba3-test-topic",
                "questions": [
                    {
                        "key": "dba3-test-001",
                        "type": "single_choice",
                        "text": "Question",
                        "answers": [
                            {
                                "key": "a",
                                "text": "A",
                                "correct": true
                            },
                            {
                                "key": "b",
                                "text": "B",
                                "correct": false
                            }
                        ],
                        "explanation": "Explanation",
                        "source": {
                            "key": "source-1",
                            "kind": "course_material",
                            "module": "DBA-3",
                            "section": "Test",
                            "locator": "slide 1",
                            "url": null
                        }
                    }
                ]
            }
            "#;

        let bundle = load_bundle(topic_json, questions_json).expect("DBA-3 should be accepted");

        assert_eq!(bundle.topic.course, "dba-3");
    }

    #[test]
    fn rejects_single_choice_with_two_correct_answers() {
        let questions_json = r#"
            {
                "schema_version": 1,
                "topic": "dba1-tools-install",
                "questions": [
                    {
                        "key": "dba1-tools-001",
                        "type": "single_choice",
                        "text": "Question",
                        "answers": [
                            {
                                "key": "a",
                                "text": "A",
                                "correct": true
                            },
                            {
                                "key": "b",
                                "text": "B",
                                "correct": true
                            }
                        ],
                        "explanation": "Explanation",
                        "source": {
                            "key": "source-1",
                            "kind": "course_material",
                            "module": "DBA-1",
                            "section": "Test",
                            "locator": "slide 1",
                            "url": null
                        }
                    }
                ]
            }
            "#;

        let result = load_bundle(TOPIC_JSON, questions_json);

        assert!(matches!(result, Err(ContentError::Validation(_))));
    }

    #[test]
    fn rejects_multiple_choice_with_only_one_correct_answer() {
        let questions_json = r#"
            {
                "schema_version": 1,
                "topic": "dba1-tools-install",
                "questions": [
                    {
                        "key": "dba1-tools-001",
                        "type": "multiple_choice",
                        "text": "Question",
                        "answers": [
                            {
                                "key": "a",
                                "text": "A",
                                "correct": true
                            },
                            {
                                "key": "b",
                                "text": "B",
                                "correct": false
                            },
                            {
                                "key": "c",
                                "text": "C",
                                "correct": false
                            }
                        ],
                        "explanation": "Explanation",
                        "source": {
                            "key": "source-1",
                            "kind": "course_material",
                            "module": "DBA-1",
                            "section": "Test",
                            "locator": "slide 1",
                            "url": null
                        }
                    }
                ]
            }
            "#;

        let result = load_bundle(TOPIC_JSON, questions_json);

        assert!(matches!(result, Err(ContentError::Validation(_))));
    }

    #[test]
    fn rejects_postgresql_docs_without_url() {
        let questions_json = r#"
            {
                "schema_version": 1,
                "topic": "dba1-tools-install",
                "questions": [
                    {
                        "key": "dba1-tools-001",
                        "type": "single_choice",
                        "text": "Question",
                        "answers": [
                            {
                                "key": "a",
                                "text": "A",
                                "correct": true
                            },
                            {
                                "key": "b",
                                "text": "B",
                                "correct": false
                            }
                        ],
                        "explanation": "Explanation",
                        "source": {
                            "key": "source-1",
                            "kind": "postgresql_docs",
                            "module": "PostgreSQL 16",
                            "section": "Test",
                            "locator": "Test",
                            "url": null
                        }
                    }
                ]
            }
            "#;

        let result = load_bundle(TOPIC_JSON, questions_json);

        assert!(matches!(result, Err(ContentError::Validation(_))));
    }

    #[test]
    fn rejects_unknown_json_field() {
        let topic_json = r#"
            {
                "schema_version": 1,
                "course": "dba-1",
                "slug": "dba1-tools-install",
                "notes_part": 1,
                "topic_number": 1,
                "title": "Tools",
                "description": null,
                "sort_order": 10,
                "unexpected": true
            }
            "#;

        let questions_json = r#"
            {
                "schema_version": 1,
                "topic": "dba1-tools-install",
                "questions": []
            }
            "#;

        let result = load_bundle(topic_json, questions_json);

        assert!(matches!(
            result,
            Err(ContentError::Json {
                document: "topic.json",
                ..
            })
        ));
    }
}
