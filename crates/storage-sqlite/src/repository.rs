use std::path::Path;

use dba_trainer_application::{RepositoryError, SessionRepository, TopicRepository};

use dba_trainer_domain::{
    AnswerOption, AnswerOptionId, AnswerResult, Question, QuestionId, QuestionType, QuizScope,
    QuizSession, SessionConfig, SessionId, SessionProgress, Source, SourceId, SourceKind,
    StatisticsFilter, StatisticsLimit, StatisticsScope, Topic, TopicId, TrainingStats,
};

use rusqlite::{Connection, OptionalExtension, params};

use crate::{StorageError, db};

pub struct SqliteRepository {
    pub(crate) connection: Connection,
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
                text
            FROM answer_options
            WHERE question_id = ?1
                AND is_active = 1
            ORDER BY sort_order, id
            ",
            )
            .map_err(repository_error)?;

        let rows = statement
            .query_map(params![question_id], |row| {
                Ok(AnswerOption {
                    id: AnswerOptionId(row.get(0)?),

                    text: row.get(1)?,
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
        let (scope, topic_id, course_code) = match &config.scope {
            QuizScope::Topic(topic_id) => ("topic", Some(topic_id.0), None),

            QuizScope::Course(course_code) => ("all", None, Some(course_code.as_str())),

            QuizScope::AllTopics => ("all", None, None),
        };

        let requested_count = config.limit.as_limit().map(|value| value as i64);

        let sql_limit = requested_count.unwrap_or(-1);

        let transaction = self.connection.transaction().map_err(repository_error)?;

        let has_active_session = transaction
            .query_row(
                "
            SELECT EXISTS(
                SELECT 1
                FROM quiz_sessions
                WHERE finished_at
                    IS NULL
                  AND cancelled_at
                    IS NULL
            )
            ",
                [],
                |row| row.get::<_, i64>(0),
            )
            .map_err(repository_error)?
            != 0;

        if has_active_session {
            return Err(RepositoryError::InvalidState(String::from(
                "unfinished quiz session already exists",
            )));
        }

        let question_ids = {
            let mut statement = transaction
                .prepare(
                    "
                    SELECT q.id
                    FROM questions q
                    JOIN topics t
                      ON t.id = q.topic_id
                    WHERE q.is_active = 1
                      AND t.is_active = 1
                      AND (
                          ?1 IS NULL
                          OR q.topic_id = ?1
                      )
                      AND (
                          ?2 IS NULL
                          OR t.course_code = ?2
                      )
                    ORDER BY RANDOM()
                    LIMIT ?3
                    ",
                )
                .map_err(repository_error)?;

            let rows = statement
                .query_map(params![topic_id, course_code, sql_limit], |row| {
                    row.get::<_, i64>(0)
                })
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
                    course_code,
                    requested_count
                )
                VALUES (?1, ?2, ?3, ?4)
                ",
                params![scope, topic_id, course_code, requested_count],
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
                  AND cancelled_at IS NULL
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
              AND finished_at IS NULL
              AND cancelled_at IS NULL
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
                    AND is_active = 1
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
            correct_option_ids: correct_ids,
            is_correct,
        })
    }

    fn active_session(&self) -> Result<Option<SessionProgress>, RepositoryError> {
        let row = self
            .connection
            .query_row(
                "
            SELECT
                s.id,
                s.scope,
                s.topic_id,
                s.course_code,
                s.current_index,

                (
                    SELECT COUNT(*)
                    FROM session_questions sq
                    WHERE sq.session_id = s.id
                ),

                (
                    SELECT COUNT(*)
                    FROM attempts a
                    WHERE a.session_id = s.id
                ),

                (
                    SELECT COALESCE(
                        SUM(a.is_correct),
                        0
                    )
                    FROM attempts a
                    WHERE a.session_id = s.id
                )

            FROM quiz_sessions s

            WHERE s.finished_at IS NULL
              AND s.cancelled_at IS NULL

            ORDER BY s.id DESC

            LIMIT 1
            ",
                [],
                |row| {
                    Ok((
                        row.get::<_, i64>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, Option<i64>>(2)?,
                        row.get::<_, Option<String>>(3)?,
                        row.get::<_, i64>(4)?,
                        row.get::<_, i64>(5)?,
                        row.get::<_, i64>(6)?,
                        row.get::<_, i64>(7)?,
                    ))
                },
            )
            .optional()
            .map_err(repository_error)?;

        let Some((
            session_id,
            scope,
            topic_id,
            course_code,
            current_index,
            total_questions,
            answered_questions,
            correct_answers,
        )) = row
        else {
            return Ok(None);
        };

        let scope = match scope.as_str() {
            "topic" => {
                let topic_id = topic_id.ok_or_else(|| {
                    RepositoryError::InvalidState(String::from("topic session has no topic_id"))
                })?;

                QuizScope::Topic(TopicId(topic_id))
            }

            "all" => match course_code {
                Some(course_code) => QuizScope::Course(course_code),
                None => QuizScope::AllTopics,
            },

            other => {
                return Err(RepositoryError::InvalidState(format!(
                    "unknown session scope: {other}"
                )));
            }
        };

        Ok(Some(SessionProgress {
            id: SessionId(session_id),

            scope,

            current_index: current_index as usize,

            total_questions: total_questions as usize,

            answered_questions: answered_questions as usize,

            correct_answers: correct_answers as usize,
        }))
    }

    fn cancel_session(&mut self, session_id: SessionId) -> Result<(), RepositoryError> {
        let changed = self
            .connection
            .execute(
                "
                UPDATE quiz_sessions
                SET cancelled_at =
                    CURRENT_TIMESTAMP
                WHERE id = ?1
                  AND finished_at IS NULL
                  AND cancelled_at IS NULL
                ",
                params![session_id.0],
            )
            .map_err(repository_error)?;

        if changed == 0 {
            return Err(RepositoryError::InvalidState(String::from(
                "session is not active",
            )));
        }

        Ok(())
    }

    fn statistics(&self, filter: &StatisticsFilter) -> Result<TrainingStats, RepositoryError> {
        let limit = match filter.limit {
            StatisticsLimit::Any => "any",
            StatisticsLimit::Twenty => "20",
            StatisticsLimit::Fifty => "50",
            StatisticsLimit::AllQuestions => "all",
        };

        let (scope, course_code, topic_id): (&str, Option<&str>, Option<i64>) = match &filter.scope
        {
            StatisticsScope::All => ("all", None, None),
            StatisticsScope::Course(course_code) => ("course", Some(course_code.as_str()), None),
            StatisticsScope::Topic(topic_id) => ("topic", None, Some(topic_id.0)),
        };

        let (completed_sessions, cancelled_sessions) = self
            .connection
            .query_row(
                "
                SELECT
                    COALESCE(SUM(
                        CASE
                            WHEN s.finished_at IS NOT NULL
                             AND s.cancelled_at IS NULL
                            THEN 1
                            ELSE 0
                        END
                    ), 0),
                    COALESCE(SUM(
                        CASE
                            WHEN s.cancelled_at IS NOT NULL
                            THEN 1
                            ELSE 0
                        END
                    ), 0)
                FROM quiz_sessions s
                WHERE (
                    ?1 = 'any'
                    OR (?1 = '20' AND s.requested_count = 20)
                    OR (?1 = '50' AND s.requested_count = 50)
                    OR (?1 = 'all' AND s.requested_count IS NULL)
                )
                AND (
                    ?2 = 'all'
                    OR (
                        ?2 = 'course'
                        AND EXISTS (
                            SELECT 1
                            FROM session_questions sq
                            JOIN questions q
                              ON q.id = sq.question_id
                            JOIN topics t
                              ON t.id = q.topic_id
                            WHERE sq.session_id = s.id
                              AND t.course_code = ?3
                        )
                    )
                    OR (
                        ?2 = 'topic'
                        AND EXISTS (
                            SELECT 1
                            FROM session_questions sq
                            JOIN questions q
                              ON q.id = sq.question_id
                            WHERE sq.session_id = s.id
                              AND q.topic_id = ?4
                        )
                    )
                )
                ",
                params![limit, scope, course_code, topic_id],
                |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?)),
            )
            .map_err(repository_error)?;

        let (answered_questions, correct_answers) = self
            .connection
            .query_row(
                "
                SELECT
                    COUNT(a.id),
                    COALESCE(SUM(a.is_correct), 0)
                FROM attempts a
                JOIN quiz_sessions s
                  ON s.id = a.session_id
                JOIN questions q
                  ON q.id = a.question_id
                JOIN topics t
                  ON t.id = q.topic_id
                WHERE s.finished_at IS NOT NULL
                  AND s.cancelled_at IS NULL
                  AND (
                      ?1 = 'any'
                      OR (?1 = '20' AND s.requested_count = 20)
                      OR (?1 = '50' AND s.requested_count = 50)
                      OR (?1 = 'all' AND s.requested_count IS NULL)
                  )
                  AND (
                      ?2 = 'all'
                      OR (?2 = 'course' AND t.course_code = ?3)
                      OR (?2 = 'topic' AND q.topic_id = ?4)
                  )
                ",
                params![limit, scope, course_code, topic_id],
                |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?)),
            )
            .map_err(repository_error)?;

        Ok(TrainingStats {
            completed_sessions: completed_sessions as usize,
            cancelled_sessions: cancelled_sessions as usize,
            answered_questions: answered_questions as usize,
            correct_answers: correct_answers as usize,
        })
    }

    fn clear_statistics(&mut self) -> Result<(), RepositoryError> {
        self.connection
            .execute(
                "
            DELETE FROM quiz_sessions
            WHERE finished_at IS NOT NULL
               OR cancelled_at IS NOT NULL
            ",
                [],
            )
            .map_err(repository_error)?;

        Ok(())
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
    fn course_session_uses_only_questions_from_selected_course() {
        let mut repository = seeded_repository();

        repository
            .connection
            .execute_batch(
                "
                INSERT INTO topics (
                    id,
                    course_code,
                    slug,
                    title,
                    description,
                    sort_order
                )
                VALUES (
                    2,
                    'dba-2',
                    'backup',
                    'Backup',
                    NULL,
                    2
                );

                INSERT INTO sources (
                    id,
                    module,
                    section,
                    locator
                )
                VALUES (
                    2,
                    'DBA-2',
                    'Backup section',
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
                    2,
                    2,
                    2,
                    'Backup question?',
                    'Backup explanation'
                );

                INSERT INTO answer_options (
                    id,
                    question_id,
                    text,
                    is_correct,
                    sort_order
                )
                VALUES
                    (3, 2, 'Correct', 1, 1),
                    (4, 2, 'Incorrect', 0, 2);
                ",
            )
            .expect("second course data should be inserted");

        let session = repository
            .start_session(&SessionConfig {
                scope: QuizScope::Course(String::from("dba-1")),
                limit: QuestionLimit::All,
            })
            .expect("course session should start");

        assert_eq!(session.question_ids, vec![QuestionId(1)]);

        let active = repository
            .active_session()
            .expect("active session should load")
            .expect("active session should exist");

        assert_eq!(active.scope, QuizScope::Course(String::from("dba-1")));
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

    #[test]
    fn finds_unfinished_session() {
        let mut repository = seeded_repository();

        let config = SessionConfig {
            scope: QuizScope::Topic(TopicId(1)),

            limit: QuestionLimit::All,
        };

        let session = repository
            .start_session(&config)
            .expect("session should start");

        let active = repository
            .active_session()
            .expect("active session should load")
            .expect("active session should exist");

        assert_eq!(active.id, session.id,);

        assert_eq!(active.current_index, 0,);

        assert_eq!(active.total_questions, 1,);

        assert_eq!(active.answered_questions, 0,);

        assert_eq!(active.correct_answers, 0,);
    }

    #[test]
    fn rejects_second_active_session() {
        let mut repository = seeded_repository();

        let config = SessionConfig {
            scope: QuizScope::Topic(TopicId(1)),

            limit: QuestionLimit::All,
        };

        repository
            .start_session(&config)
            .expect("first session should start");

        let error = repository
            .start_session(&config)
            .expect_err("second active session must be rejected");

        assert!(matches!(error, RepositoryError::InvalidState(_)));
    }

    #[test]
    fn cancelled_session_is_not_resumable() {
        let mut repository = seeded_repository();

        let config = SessionConfig {
            scope: QuizScope::Topic(TopicId(1)),

            limit: QuestionLimit::All,
        };

        let session = repository
            .start_session(&config)
            .expect("session should start");

        repository
            .cancel_session(session.id)
            .expect("session should cancel");

        let active = repository
            .active_session()
            .expect("active session query should work");

        assert_eq!(active, None,);

        let stats = repository
            .statistics(&StatisticsFilter::all())
            .expect("statistics should load");

        assert_eq!(stats.cancelled_sessions, 1,);

        assert_eq!(stats.completed_sessions, 0,);
    }

    #[test]
    fn completed_session_is_counted_in_statistics() {
        let mut repository = seeded_repository();

        let config = SessionConfig {
            scope: QuizScope::Topic(TopicId(1)),

            limit: QuestionLimit::All,
        };

        let session = repository
            .start_session(&config)
            .expect("session should start");

        let question = repository
            .current_question(session.id)
            .expect("question should load")
            .expect("question should exist");

        repository
            .submit_answer(session.id, question.id, &[AnswerOptionId(1)])
            .expect("answer should be accepted");

        let stats = repository
            .statistics(&StatisticsFilter::all())
            .expect("statistics should load");

        assert_eq!(stats.completed_sessions, 1,);

        assert_eq!(stats.cancelled_sessions, 0,);

        assert_eq!(stats.answered_questions, 1,);

        assert_eq!(stats.correct_answers, 1,);

        assert_eq!(stats.incorrect_answers(), 0,);

        assert_eq!(stats.accuracy_percent(), 100.0,);
    }

    #[test]
    fn filters_statistics_by_mode_course_and_topic() {
        let mut repository = seeded_repository();

        let config = SessionConfig {
            scope: QuizScope::Topic(TopicId(1)),
            limit: QuestionLimit::Twenty,
        };

        let session = repository
            .start_session(&config)
            .expect("session should start");

        let question = repository
            .current_question(session.id)
            .expect("question should load")
            .expect("question should exist");

        repository
            .submit_answer(session.id, question.id, &[AnswerOptionId(1)])
            .expect("answer should complete session");

        let twenty = repository
            .statistics(&StatisticsFilter {
                scope: StatisticsScope::All,
                limit: StatisticsLimit::Twenty,
            })
            .expect("20-question statistics should load");

        assert_eq!(twenty.completed_sessions, 1);
        assert_eq!(twenty.answered_questions, 1);

        let fifty = repository
            .statistics(&StatisticsFilter {
                scope: StatisticsScope::All,
                limit: StatisticsLimit::Fifty,
            })
            .expect("50-question statistics should load");

        assert_eq!(fifty.completed_sessions, 0);
        assert_eq!(fifty.answered_questions, 0);

        let course = repository
            .statistics(&StatisticsFilter {
                scope: StatisticsScope::Course(String::from("dba-1")),
                limit: StatisticsLimit::Any,
            })
            .expect("course statistics should load");

        assert_eq!(course.completed_sessions, 1);
        assert_eq!(course.correct_answers, 1);

        let topic = repository
            .statistics(&StatisticsFilter {
                scope: StatisticsScope::Topic(TopicId(1)),
                limit: StatisticsLimit::Any,
            })
            .expect("topic statistics should load");

        assert_eq!(topic.completed_sessions, 1);
        assert_eq!(topic.correct_answers, 1);
    }

    #[test]
    fn clear_statistics_removes_history_but_keeps_active_session() {
        let mut repository = seeded_repository();

        let completed_config = SessionConfig {
            scope: QuizScope::Topic(TopicId(1)),
            limit: QuestionLimit::All,
        };

        let completed_session = repository
            .start_session(&completed_config)
            .expect("completed session should start");

        let question = repository
            .current_question(completed_session.id)
            .expect("question should load")
            .expect("question should exist");

        repository
            .submit_answer(completed_session.id, question.id, &[AnswerOptionId(1)])
            .expect("answer should complete session");

        let active_session = repository
            .start_session(&completed_config)
            .expect("active session should start");

        repository
            .clear_statistics()
            .expect("statistics should clear");

        let stats = repository
            .statistics(&StatisticsFilter::all())
            .expect("statistics should load");

        assert_eq!(stats.completed_sessions, 0);
        assert_eq!(stats.cancelled_sessions, 0);
        assert_eq!(stats.answered_questions, 0);
        assert_eq!(stats.correct_answers, 0);

        let active = repository
            .active_session()
            .expect("active session should load")
            .expect("active session should remain");

        assert_eq!(active.id, active_session.id);
    }
}
