use dba_trainer_application::{ContentLocaleRepository, RepositoryError};
use dba_trainer_content::{
    AnswerDocument, ContentBundle, QuestionDocument, QuestionTypeDocument, SourceDocument,
    SourceKindDocument, TopicDocument, load_builtin_bundles_for_locale,
};

use rusqlite::{Transaction, params};

use crate::{StorageError, repository::SqliteRepository};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentSyncReport {
    pub topic_slug: String,
    pub questions: usize,
    pub answers: usize,
}

impl SqliteRepository {
    pub fn sync_bundle(
        &mut self,
        bundle: &ContentBundle,
    ) -> Result<ContentSyncReport, StorageError> {
        let transaction = self.connection.transaction()?;

        let topic_id = sync_topic(&transaction, &bundle.topic)?;

        transaction.execute(
            "
            UPDATE questions
            SET is_active = 0
            WHERE topic_id = ?1
              AND content_key IS NOT NULL
            ",
            params![topic_id],
        )?;

        let mut answer_count = 0;

        for question in &bundle.questions.questions {
            let source_id = sync_source(&transaction, &question.source)?;

            let question_id = sync_question(&transaction, topic_id, source_id, question)?;

            transaction.execute(
                "
                UPDATE answer_options
                SET is_active = 0
                WHERE question_id = ?1
                  AND content_key IS NOT NULL
                ",
                params![question_id],
            )?;

            for (index, answer) in question.answers.iter().enumerate() {
                sync_answer(&transaction, question_id, answer, index as i64)?;

                answer_count += 1;
            }
        }

        transaction.commit()?;

        Ok(ContentSyncReport {
            topic_slug: bundle.topic.slug.clone(),

            questions: bundle.questions.questions.len(),

            answers: answer_count,
        })
    }
}

impl ContentLocaleRepository for SqliteRepository {
    fn set_content_locale(&mut self, locale: &str) -> Result<(), RepositoryError> {
        let bundles = load_builtin_bundles_for_locale(locale)
            .map_err(|error| RepositoryError::Storage(error.to_string()))?;

        for bundle in &bundles {
            self.sync_bundle(bundle)
                .map_err(|error| RepositoryError::Storage(error.to_string()))?;
        }

        Ok(())
    }
}

fn sync_topic(transaction: &Transaction<'_>, topic: &TopicDocument) -> rusqlite::Result<i64> {
    transaction.execute(
        "
        INSERT INTO topics (
            slug,
            title,
            description,
            notes_part,
            topic_number,
            sort_order,
            is_active,
            course_code,
            content_key
        )
        VALUES (
            ?1,
            ?2,
            ?3,
            ?4,
            ?5,
            ?6,
            1,
            ?7,
            ?8
        )

        ON CONFLICT(content_key)
        DO UPDATE SET
            slug = excluded.slug,
            title = excluded.title,
            description =
                excluded.description,
            notes_part =
                excluded.notes_part,
            topic_number =
                excluded.topic_number,
            sort_order =
                excluded.sort_order,
            is_active = 1,
            course_code =
                excluded.course_code
        ",
        params![
            topic.slug,
            topic.title,
            topic.description,
            topic.notes_part,
            topic.topic_number,
            topic.sort_order,
            topic.course,
            topic.slug,
        ],
    )?;

    transaction.query_row(
        "
        SELECT id
        FROM topics
        WHERE content_key = ?1
        ",
        params![topic.slug],
        |row| row.get(0),
    )
}

fn sync_source(transaction: &Transaction<'_>, source: &SourceDocument) -> rusqlite::Result<i64> {
    transaction.execute(
        "
        INSERT INTO sources (
            module,
            section,
            locator,
            kind,
            url,
            content_key
        )
        VALUES (
            ?1,
            ?2,
            ?3,
            ?4,
            ?5,
            ?6
        )

        ON CONFLICT(content_key)
        DO UPDATE SET
            module =
                excluded.module,
            section =
                excluded.section,
            locator =
                excluded.locator,
            kind =
                excluded.kind,
            url =
                excluded.url
        ",
        params![
            source.module,
            source.section,
            source.locator,
            source_kind_value(source.kind),
            source.url,
            source.key,
        ],
    )?;

    transaction.query_row(
        "
        SELECT id
        FROM sources
        WHERE content_key = ?1
        ",
        params![source.key],
        |row| row.get(0),
    )
}

fn sync_question(
    transaction: &Transaction<'_>,
    topic_id: i64,
    source_id: i64,
    question: &QuestionDocument,
) -> rusqlite::Result<i64> {
    transaction.execute(
        "
        INSERT INTO questions (
            topic_id,
            source_id,
            text,
            explanation,
            is_active,
            question_type,
            content_key
        )
        VALUES (
            ?1,
            ?2,
            ?3,
            ?4,
            1,
            ?5,
            ?6
        )

        ON CONFLICT(content_key)
        DO UPDATE SET
            topic_id =
                excluded.topic_id,
            source_id =
                excluded.source_id,
            text =
                excluded.text,
            explanation =
                excluded.explanation,
            is_active = 1,
            question_type =
                excluded.question_type
        ",
        params![
            topic_id,
            source_id,
            question.text,
            question.explanation,
            question_type_value(question.question_type),
            question.key,
        ],
    )?;

    transaction.query_row(
        "
        SELECT id
        FROM questions
        WHERE content_key = ?1
        ",
        params![question.key],
        |row| row.get(0),
    )
}

fn sync_answer(
    transaction: &Transaction<'_>,
    question_id: i64,
    answer: &AnswerDocument,
    sort_order: i64,
) -> rusqlite::Result<()> {
    transaction.execute(
        "
        INSERT INTO answer_options (
            question_id,
            text,
            is_correct,
            sort_order,
            content_key,
            is_active
        )
        VALUES (
            ?1,
            ?2,
            ?3,
            ?4,
            ?5,
            1
        )

        ON CONFLICT(
            question_id,
            content_key
        )
        DO UPDATE SET
            text =
                excluded.text,
            is_correct =
                excluded.is_correct,
            sort_order =
                excluded.sort_order,
            is_active = 1
        ",
        params![
            question_id,
            answer.text,
            answer.correct as i64,
            sort_order,
            answer.key,
        ],
    )?;

    Ok(())
}

fn question_type_value(question_type: QuestionTypeDocument) -> &'static str {
    match question_type {
        QuestionTypeDocument::SingleChoice => "single_choice",

        QuestionTypeDocument::MultipleChoice => "multiple_choice",
    }
}

fn source_kind_value(source_kind: SourceKindDocument) -> &'static str {
    match source_kind {
        SourceKindDocument::CourseMaterial => "course_material",

        SourceKindDocument::PostgreSqlDocs => "postgresql_docs",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use dba_trainer_content::load_bundle;

    #[test]
    fn sync_is_idempotent() {
        let topic_json = r#"
        {
            "schema_version": 1,
            "course": "dba-1",
            "notes_part": 1,
            "topic_number": 1,
            "slug": "dba1-test",
            "title": "Test",
            "description": null,
            "sort_order": 10
        }
        "#;

        let questions_json = r#"
        {
            "schema_version": 1,
            "topic": "dba1-test",
            "questions": [
                {
                    "key": "dba1-test-001",
                    "type": "single_choice",
                    "text": "Question?",
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
                        "key": "test-source",
                        "kind": "course_material",
                        "module": "DBA-1",
                        "section": "Test",
                        "locator": "Test",
                        "url": null
                    }
                }
            ]
        }
        "#;

        let bundle = load_bundle(topic_json, questions_json).expect("content should be valid");

        let mut repository = SqliteRepository::in_memory().expect("database should open");

        repository
            .sync_bundle(&bundle)
            .expect("first sync should work");

        repository
            .sync_bundle(&bundle)
            .expect("second sync should work");

        let topics: i64 = repository
            .connection
            .query_row(
                "
                    SELECT COUNT(*)
                    FROM topics
                    WHERE content_key =
                        'dba1-test'
                    ",
                [],
                |row| row.get(0),
            )
            .expect("topic count should load");

        let questions: i64 = repository
            .connection
            .query_row(
                "
                    SELECT COUNT(*)
                    FROM questions
                    WHERE content_key =
                        'dba1-test-001'
                    ",
                [],
                |row| row.get(0),
            )
            .expect("question count should load");

        let answers: i64 = repository
            .connection
            .query_row(
                "
                    SELECT COUNT(*)
                    FROM answer_options
                    WHERE is_active = 1
                    ",
                [],
                |row| row.get(0),
            )
            .expect("answer count should load");

        assert_eq!(topics, 1);
        assert_eq!(questions, 1);
        assert_eq!(answers, 2);
    }
}
