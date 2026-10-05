use std::path::Path;

use dba_trainer_application::{RepositoryError, SessionRepository, TopicRepository};

use dba_trainer_domain::{
    AnswerOption, AnswerOptionId, AnswerResult, Question, QuestionId, QuestionType, QuizScope,
    QuizSession, SessionConfig, SessionId, Source, SourceId, SourceKind, Topic, TopicId,
};

use rusqlite::{Connection, OptionalExtension, params};

use crate::{StorageError, db};

pub struct SqliteRepository {
    connection: Connection,
}

impl SqliteRepository {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, StorageError> {
        Ok(Self {
            connection: db::open(path)?,
        })
    }

    pub fn in_memory() -> Result<Self, StorageError> {
        Ok(Self {
            connection: db::open_in_memory()?,
        })
    }

    fn load_question(&self, question_id: QuestionId) -> Result<Question, RepositoryError> {
        let row = self
            .connection
            .query_row(
                "
            SELECT
                q.id,
                q.topic_id,
                q.question_type,
                q.text,
                q.explanation,

                s.id,
                s.kind,
                s.module,
                s.section,
                s.locator,
                s.url

            FROM questions q

            JOIN sources s
              ON s.id = q.source_id

            WHERE q.id = ?1
            ",
                params![question_id.0],
                |row| {
                    Ok((
                        row.get::<_, i64>(0)?,
                        row.get::<_, i64>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, String>(3)?,
                        row.get::<_, String>(4)?,
                        row.get::<_, i64>(5)?,
                        row.get::<_, String>(6)?,
                        row.get::<_, String>(7)?,
                        row.get::<_, String>(8)?,
                        row.get::<_, String>(9)?,
                        row.get::<_, Option<String>>(10)?,
                    ))
                },
            )
            .optional()
            .map_err(repository_error)?;

        let Some((
            question_id,
            topic_id,
            question_type,
            text,
            explanation,
            source_id,
            source_kind,
            module,
            section,
            locator,
            url,
        )) = row
        else {
            return Err(RepositoryError::NotFound);
        };

        let question_type = parse_question_type(&question_type)?;

        let source_kind = parse_source_kind(&source_kind)?;

        let mut statement = self
            .connection
            .prepare(
                "
            SELECT
                id,
                text,
                is_correct

            FROM answer_options

            WHERE question_id = ?1

            ORDER BY sort_order, id
            ",
            )
            .map_err(repository_error)?;

        let rows = statement
            .query_map(params![question_id], |row| {
                Ok(AnswerOption {
                    id: AnswerOptionId(row.get(0)?),

                    text: row.get(1)?,

                    is_correct: row.get::<_, i64>(2)? != 0,
                })
            })
            .map_err(repository_error)?;

        let options = rows
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(repository_error)?;

        Ok(Question {
            id: QuestionId(question_id),
            topic_id: TopicId(topic_id),

            question_type,

            text,
            explanation,

            source: Source {
                id: SourceId(source_id),
                kind: source_kind,
                module,
                section,
                locator,
                url,
            },

            options,
        })
    }
}

fn repository_error(error: rusqlite::Error) -> RepositoryError {
    RepositoryError::Storage(error.to_string())
}

fn parse_question_type(value: &str) -> Result<QuestionType, RepositoryError> {
    match value {
        "single_choice" => Ok(QuestionType::SingleChoice),

        "multiple_choice" => Ok(QuestionType::MultipleChoice),

        other => Err(RepositoryError::Storage(format!(
            "unknown question type: {other}"
        ))),
    }
}

fn parse_source_kind(value: &str) -> Result<SourceKind, RepositoryError> {
    match value {
        "course_material" => Ok(SourceKind::CourseMaterial),

        "postgresql_docs" => Ok(SourceKind::PostgreSqlDocs),

        other => Err(RepositoryError::Storage(format!(
            "unknown source kind: {other}"
        ))),
    }
}

impl TopicRepository for SqliteRepository {
    fn topics(&self) -> Result<Vec<Topic>, RepositoryError> {
        let mut statement = self
            .connection
            .prepare(
                "
                    SELECT
                        id,
                        course_code,
                        slug,
                        title,
                        description
                    FROM topics
                    WHERE is_active = 1
                    ORDER BY sort_order, title
                ",
            )
            .map_err(repository_error)?;

        let rows = statement
            .query_map([], |row| {
                Ok(Topic {
                    id: TopicId(row.get(0)?),
                    course_code: row.get(1)?,
                    slug: row.get(2)?,
                    title: row.get(3)?,
                    description: row.get(4)?,
                })
            })
            .map_err(repository_error)?;

        rows.collect::<rusqlite::Result<Vec<_>>>()
            .map_err(repository_error)
    }
}

impl SessionRepository for SqliteRepository {
    fn start_session(&mut self, config: &SessionConfig) -> Result<QuizSession, RepositoryError> {
        let (scope, topic_id) = match config.scope {
            QuizScope::Topic(topic_id) => ("topic", Some(topic_id.0)),

            QuizScope::AllTopics => ("all", None),
        };

        let requested_count = config.limit.as_limit().map(|value| value as i64);

        let sql_limit = requested_count.unwrap_or(-1);

        let transaction = self.connection.transaction().map_err(repository_error)?;

        let question_ids = {
            let mut statement = transaction
                .prepare(
                    "
                    SELECT id
                    FROM questions
                    WHERE is_active = 1
                      AND (
                          ?1 IS NULL
                          OR topic_id = ?1
                      )
                    ORDER BY RANDOM()
                    LIMIT ?2
                    ",
                )
                .map_err(repository_error)?;

            let rows = statement
                .query_map(params![topic_id, sql_limit], |row| row.get::<_, i64>(0))
                .map_err(repository_error)?;

            let ids = rows
                .collect::<rusqlite::Result<Vec<_>>>()
                .map_err(repository_error)?;

            ids.into_iter().map(QuestionId).collect::<Vec<_>>()
        };

        if question_ids.is_empty() {
            return Err(RepositoryError::NoQuestions);
        }

        transaction
            .execute(
                "
                INSERT INTO quiz_sessions (
                    scope,
                    topic_id,
                    requested_count
                )
                VALUES (?1, ?2, ?3)
                ",
                params![scope, topic_id, requested_count],
            )
            .map_err(repository_error)?;

        let session_id = SessionId(transaction.last_insert_rowid());

        for (position, question_id) in question_ids.iter().enumerate() {
            transaction
                .execute(
                    "
                    INSERT INTO session_questions (
                        session_id,
                        question_id,
                        position
                    )
                    VALUES (?1, ?2, ?3)
                    ",
                    params![session_id.0, question_id.0, position as i64],
                )
                .map_err(repository_error)?;
        }

        transaction.commit().map_err(repository_error)?;

        Ok(QuizSession {
            id: session_id,
            question_ids,
            current_index: 0,
        })
    }

    fn current_question(&self, session_id: SessionId) -> Result<Option<Question>, RepositoryError> {
        let current_index = self
            .connection
            .query_row(
                "
                SELECT current_index
                FROM quiz_sessions
                WHERE id = ?1
                ",
                params![session_id.0],
                |row| row.get::<_, i64>(0),
            )
            .optional()
            .map_err(repository_error)?;

        let Some(current_index) = current_index else {
            return Err(RepositoryError::NotFound);
        };

        let question_id = self
            .connection
            .query_row(
                "
                SELECT question_id
                FROM session_questions
                WHERE session_id = ?1
                  AND position = ?2
                ",
                params![session_id.0, current_index],
                |row| row.get::<_, i64>(0),
            )
            .optional()
            .map_err(repository_error)?;

        match question_id {
            Some(question_id) => self.load_question(QuestionId(question_id)).map(Some),

            None => Ok(None),
        }
    }

    fn submit_answer(
        &mut self,
        session_id: SessionId,
        question_id: QuestionId,
        answer_option_ids: &[AnswerOptionId],
    ) -> Result<AnswerResult, RepositoryError> {
        let transaction = self.connection.transaction().map_err(repository_error)?;

        let current_index = transaction
            .query_row(
                "
            SELECT current_index
            FROM quiz_sessions
            WHERE id = ?1
            ",
                params![session_id.0],
                |row| row.get::<_, i64>(0),
            )
            .optional()
            .map_err(repository_error)?;

        let Some(current_index) = current_index else {
            return Err(RepositoryError::NotFound);
        };

        let expected_question_id = transaction
            .query_row(
                "
            SELECT question_id
            FROM session_questions
            WHERE session_id = ?1
              AND position = ?2
            ",
                params![session_id.0, current_index],
                |row| row.get::<_, i64>(0),
            )
            .optional()
            .map_err(repository_error)?;

        let Some(expected_question_id) = expected_question_id else {
            return Err(RepositoryError::InvalidState(String::from(
                "session is already finished",
            )));
        };

        if expected_question_id != question_id.0 {
            return Err(RepositoryError::InvalidState(String::from(
                "question is not current",
            )));
        }

        let question_type = transaction
            .query_row(
                "
            SELECT question_type
            FROM questions
            WHERE id = ?1
            ",
                params![question_id.0],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .map_err(repository_error)?;

        let Some(question_type) = question_type else {
            return Err(RepositoryError::NotFound);
        };

        let question_type = parse_question_type(&question_type)?;

        let mut selected_ids = answer_option_ids.to_vec();

        selected_ids.sort_by_key(|id| id.0);

        let original_len = selected_ids.len();

        selected_ids.dedup();

        if selected_ids.len() != original_len {
            return Err(RepositoryError::InvalidAnswerSelection(String::from(
                "duplicate answer options",
            )));
        }

        if selected_ids.is_empty() {
            return Err(RepositoryError::InvalidAnswerSelection(String::from(
                "at least one answer must be selected",
            )));
        }

        if question_type == QuestionType::SingleChoice && selected_ids.len() != 1 {
            return Err(RepositoryError::InvalidAnswerSelection(String::from(
                "single-choice question requires exactly one answer",
            )));
        }

        let options = {
            let mut statement = transaction
                .prepare(
                    "
                SELECT
                    id,
                    is_correct
                FROM answer_options
                WHERE question_id = ?1
                ",
                )
                .map_err(repository_error)?;

            let rows = statement
                .query_map(params![question_id.0], |row| {
                    Ok((
                        AnswerOptionId(row.get::<_, i64>(0)?),
                        row.get::<_, i64>(1)? != 0,
                    ))
                })
                .map_err(repository_error)?;

            rows.collect::<rusqlite::Result<Vec<_>>>()
                .map_err(repository_error)?
        };

        for selected_id in &selected_ids {
            if !options.iter().any(|(id, _)| id == selected_id) {
                return Err(RepositoryError::InvalidAnswerSelection(String::from(
                    "answer option does not belong to question",
                )));
            }
        }

        let mut correct_ids = options
            .iter()
            .filter(|(_, is_correct)| *is_correct)
            .map(|(id, _)| *id)
            .collect::<Vec<_>>();

        correct_ids.sort_by_key(|id| id.0);

        let is_correct = selected_ids == correct_ids;

        transaction
            .execute(
                "
            INSERT INTO attempts (
                session_id,
                question_id,
                is_correct
            )
            VALUES (?1, ?2, ?3)
            ",
                params![session_id.0, question_id.0, is_correct as i64],
            )
            .map_err(repository_error)?;

        let attempt_id = transaction.last_insert_rowid();

        for answer_option_id in &selected_ids {
            transaction
                .execute(
                    "
                INSERT INTO attempt_answers (
                    attempt_id,
                    question_id,
                    answer_option_id
                )
                VALUES (?1, ?2, ?3)
                ",
                    params![attempt_id, question_id.0, answer_option_id.0],
                )
                .map_err(repository_error)?;
        }

        transaction
            .execute(
                "
            UPDATE quiz_sessions
            SET current_index =
                current_index + 1
            WHERE id = ?1
            ",
                params![session_id.0],
            )
            .map_err(repository_error)?;

        transaction
            .execute(
                "
            UPDATE quiz_sessions
            SET finished_at =
                CURRENT_TIMESTAMP
            WHERE id = ?1
              AND current_index >= (
                  SELECT COUNT(*)
                  FROM session_questions
                  WHERE session_id = ?1
              )
            ",
                params![session_id.0],
            )
            .map_err(repository_error)?;

        transaction.commit().map_err(repository_error)?;

        Ok(AnswerResult {
            question_id,
            selected_option_ids: selected_ids,
            is_correct,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use dba_trainer_domain::{QuestionLimit, QuizScope};

    fn seeded_repository() -> SqliteRepository {
        let repository = SqliteRepository::in_memory().expect("in-memory database should open");

        repository
            .connection
            .execute_batch(
                "
                INSERT INTO topics (
                    id,
                    slug,
                    title,
                    description,
                    sort_order
                )
                VALUES (
                    1,
                    'architecture',
                    'Architecture',
                    NULL,
                    1
                );

                INSERT INTO sources (
                    id,
                    module,
                    section,
                    locator
                )
                VALUES (
                    1,
                    'DBA-1',
                    'Test section',
                    'page 1'
                );

                INSERT INTO questions (
                    id,
                    topic_id,
                    source_id,
                    text,
                    explanation
                )
                VALUES (
                    1,
                    1,
                    1,
                    'Test question?',
                    'Test explanation'
                );

                INSERT INTO answer_options (
                    id,
                    question_id,
                    text,
                    is_correct,
                    sort_order
                )
                VALUES
                    (1, 1, 'Correct', 1, 1),
                    (2, 1, 'Incorrect', 0, 2);
                ",
            )
            .expect("test data should be inserted");

        repository
    }

    #[test]
    fn loads_topics_from_sqlite() {
        let repository = seeded_repository();

        let topics = repository.topics().expect("topics should load");

        assert_eq!(topics.len(), 1);
        assert_eq!(topics[0].id, TopicId(1));
        assert_eq!(topics[0].slug, "architecture");
    }

    #[test]
    fn completes_single_question_session() {
        let mut repository = seeded_repository();

        let config = SessionConfig {
            scope: QuizScope::Topic(TopicId(1)),
            limit: QuestionLimit::Twenty,
        };

        let session = repository
            .start_session(&config)
            .expect("session should start");

        assert_eq!(session.total_questions(), 1);

        let question = repository
            .current_question(session.id)
            .expect("question should load")
            .expect("question should exist");

        assert_eq!(question.id, QuestionId(1));
        assert_eq!(question.options.len(), 2);
        assert_eq!(question.question_type, QuestionType::SingleChoice);

        let result = repository
            .submit_answer(session.id, question.id, &[AnswerOptionId(1)])
            .expect("answer should be accepted");

        assert!(result.is_correct);
        assert_eq!(result.selected_option_ids, vec![AnswerOptionId(1)]);

        let next_question = repository
            .current_question(session.id)
            .expect("session should load");

        assert!(next_question.is_none());
    }
}
