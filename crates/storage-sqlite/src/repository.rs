use std::path::Path;

use dba_trainer_application::{
    LabRepository, LearningHistoryRepository, RepositoryError, SessionRepository, TopicRepository,
};

use dba_trainer_domain::{
    ActivityStats, AnswerOption, AnswerOptionId, AnswerResult, CourseProgress, DailyActivity, Lab,
    Question, QuestionAttemptHistory, QuestionId, QuestionMistakeDetail, QuestionMistakeSummary,
    QuestionType, QuizScope, QuizSession, STUDY_DAY_QUESTION_TARGET, SessionConfig, SessionId,
    SessionProgress, Source, SourceId, SourceKind, StatisticsFilter, StatisticsLimit,
    StatisticsScope, TOPIC_PASS_THRESHOLD_PERCENT, Topic, TopicId, TopicMasteryStats,
    TrainingStats,
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
                        notes_part,
                        topic_number,
                        title,
                        description
                    FROM topics
                    WHERE is_active = 1
                    ORDER BY course_code, topic_number, sort_order, title
                ",
            )
            .map_err(repository_error)?;

        let rows = statement
            .query_map([], |row| {
                Ok(Topic {
                    id: TopicId(row.get(0)?),
                    course_code: row.get(1)?,
                    slug: row.get(2)?,
                    notes_part: row.get(3)?,
                    topic_number: row.get(4)?,
                    title: row.get(5)?,
                    description: row.get(6)?,
                })
            })
            .map_err(repository_error)?;

        rows.collect::<rusqlite::Result<Vec<_>>>()
            .map_err(repository_error)
    }

    fn topic_mastery(&self, topic_id: TopicId) -> Result<TopicMasteryStats, RepositoryError> {
        let mut statement = self
            .connection
            .prepare(
                "
                SELECT
                    s.id,
                    COUNT(a.id),
                    COALESCE(SUM(a.is_correct), 0)
                FROM quiz_sessions s
                LEFT JOIN attempts a
                  ON a.session_id = s.id
                WHERE s.scope = 'topic'
                  AND s.topic_id = ?1
                  AND s.finished_at IS NOT NULL
                  AND s.cancelled_at IS NULL
                GROUP BY s.id
                ORDER BY s.finished_at, s.id
                ",
            )
            .map_err(repository_error)?;

        let rows = statement
            .query_map(params![topic_id.0], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, i64>(2)?,
                ))
            })
            .map_err(repository_error)?;

        let attempts = rows
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(repository_error)?;

        let mut best_correct_answers = 0usize;
        let mut best_answered_questions = 0usize;
        let mut attempts_to_pass = None;

        for (index, (_, answered, correct)) in attempts.iter().enumerate() {
            let answered = *answered as usize;
            let correct = *correct as usize;

            if answered > 0
                && (best_answered_questions == 0
                    || correct * best_answered_questions > best_correct_answers * answered)
            {
                best_correct_answers = correct;
                best_answered_questions = answered;
            }

            if attempts_to_pass.is_none()
                && answered > 0
                && (correct as f64 / answered as f64) * 100.0 >= TOPIC_PASS_THRESHOLD_PERCENT
            {
                attempts_to_pass = Some(index + 1);
            }
        }

        Ok(TopicMasteryStats {
            topic_id,
            completed_attempts: attempts.len(),
            best_correct_answers,
            best_answered_questions,
            attempts_to_pass,
        })
    }
}

impl LabRepository for SqliteRepository {
    fn labs(&self) -> Result<Vec<Lab>, RepositoryError> {
        let mut statement = self
            .connection
            .prepare(
                "
                SELECT
                    id,
                    course_code,
                    content_key,
                    lab_number,
                    title,
                    description,
                    body
                FROM labs
                WHERE is_active = 1
                ORDER BY course_code, sort_order, lab_number, title
                ",
            )
            .map_err(repository_error)?;

        let rows = statement
            .query_map([], |row| {
                Ok(Lab {
                    id: row.get(0)?,
                    course_code: row.get(1)?,
                    key: row.get(2)?,
                    lab_number: row.get(3)?,
                    title: row.get(4)?,
                    description: row.get(5)?,
                    body: row.get(6)?,
                })
            })
            .map_err(repository_error)?;

        rows.collect::<rusqlite::Result<Vec<_>>>()
            .map_err(repository_error)
    }
}

impl LearningHistoryRepository for SqliteRepository {
    fn mistake_summaries(&self) -> Result<Vec<QuestionMistakeSummary>, RepositoryError> {
        let mut statement = self
            .connection
            .prepare(
                "
                SELECT
                    q.id,
                    q.topic_id,
                    t.course_code,
                    t.title,
                    q.text,
                    COUNT(a.id),
                    SUM(CASE WHEN a.is_correct = 0 THEN 1 ELSE 0 END),
                    MAX(CASE WHEN a.is_correct = 0 THEN a.answered_at END)
                FROM attempts a
                JOIN questions q
                  ON q.id = a.question_id
                JOIN topics t
                  ON t.id = q.topic_id
                WHERE q.is_active = 1
                  AND t.is_active = 1
                GROUP BY q.id, q.topic_id, t.course_code, t.title, q.text
                HAVING SUM(CASE WHEN a.is_correct = 0 THEN 1 ELSE 0 END) > 0
                ORDER BY
                    SUM(CASE WHEN a.is_correct = 0 THEN 1 ELSE 0 END) DESC,
                    COUNT(a.id) DESC,
                    MAX(CASE WHEN a.is_correct = 0 THEN a.answered_at END) DESC,
                    q.id
                ",
            )
            .map_err(repository_error)?;

        let rows = statement
            .query_map([], |row| {
                Ok(QuestionMistakeSummary {
                    question_id: QuestionId(row.get(0)?),
                    topic_id: TopicId(row.get(1)?),
                    course_code: row.get(2)?,
                    topic_title: row.get(3)?,
                    question_text: row.get(4)?,
                    total_attempts: row.get::<_, i64>(5)? as usize,
                    incorrect_attempts: row.get::<_, i64>(6)? as usize,
                    last_incorrect_at: row.get(7)?,
                })
            })
            .map_err(repository_error)?;

        rows.collect::<rusqlite::Result<Vec<_>>>()
            .map_err(repository_error)
    }

    fn mistake_detail(
        &self,
        question_id: QuestionId,
    ) -> Result<QuestionMistakeDetail, RepositoryError> {
        let summary = self
            .mistake_summaries()?
            .into_iter()
            .find(|summary| summary.question_id == question_id)
            .ok_or(RepositoryError::NotFound)?;

        let question = self.load_question(question_id)?;

        let correct_answers = {
            let mut statement = self
                .connection
                .prepare(
                    "
                    SELECT text
                    FROM answer_options
                    WHERE question_id = ?1
                      AND is_active = 1
                      AND is_correct = 1
                    ORDER BY sort_order, id
                    ",
                )
                .map_err(repository_error)?;

            let rows = statement
                .query_map(params![question_id.0], |row| row.get::<_, String>(0))
                .map_err(repository_error)?;

            rows.collect::<rusqlite::Result<Vec<_>>>()
                .map_err(repository_error)?
        };

        let attempts = {
            let mut statement = self
                .connection
                .prepare(
                    "
                    SELECT id, answered_at, is_correct
                    FROM attempts
                    WHERE question_id = ?1
                    ORDER BY answered_at DESC, id DESC
                    ",
                )
                .map_err(repository_error)?;

            let rows = statement
                .query_map(params![question_id.0], |row| {
                    Ok((
                        row.get::<_, i64>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, i64>(2)? != 0,
                    ))
                })
                .map_err(repository_error)?;

            rows.collect::<rusqlite::Result<Vec<_>>>()
                .map_err(repository_error)?
        };

        let mut history = Vec::with_capacity(attempts.len());

        for (attempt_id, answered_at, is_correct) in attempts {
            let mut statement = self
                .connection
                .prepare(
                    "
                    SELECT ao.text
                    FROM attempt_answers aa
                    JOIN answer_options ao
                      ON ao.id = aa.answer_option_id
                     AND ao.question_id = aa.question_id
                    WHERE aa.attempt_id = ?1
                    ORDER BY ao.sort_order, ao.id
                    ",
                )
                .map_err(repository_error)?;

            let rows = statement
                .query_map(params![attempt_id], |row| row.get::<_, String>(0))
                .map_err(repository_error)?;

            let selected_answers = rows
                .collect::<rusqlite::Result<Vec<_>>>()
                .map_err(repository_error)?;

            history.push(QuestionAttemptHistory {
                answered_at,
                is_correct,
                selected_answers,
            });
        }

        Ok(QuestionMistakeDetail {
            summary,
            question,
            correct_answers,
            attempts: history,
        })
    }

    fn activity_stats(&self) -> Result<ActivityStats, RepositoryError> {
        let (active_days, study_days, total_answered_questions) = self
            .connection
            .query_row(
                "
                SELECT
                    COALESCE(SUM(CASE WHEN open_count > 0 THEN 1 ELSE 0 END), 0),
                    COALESCE(SUM(CASE WHEN answered_questions >= ?1 THEN 1 ELSE 0 END), 0),
                    COALESCE(SUM(answered_questions), 0)
                FROM daily_activity
                ",
                params![STUDY_DAY_QUESTION_TARGET as i64],
                |row| {
                    Ok((
                        row.get::<_, i64>(0)?,
                        row.get::<_, i64>(1)?,
                        row.get::<_, i64>(2)?,
                    ))
                },
            )
            .map_err(repository_error)?;

        let (today_answered_questions, today_correct_answers) = self
            .connection
            .query_row(
                "
                SELECT
                    COALESCE(answered_questions, 0),
                    COALESCE(correct_answers, 0)
                FROM daily_activity
                WHERE activity_date = date('now', 'localtime')
                ",
                [],
                |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?)),
            )
            .optional()
            .map_err(repository_error)?
            .unwrap_or((0, 0));

        let last_7_days_answered_questions = self
            .connection
            .query_row(
                "
                SELECT COALESCE(SUM(answered_questions), 0)
                FROM daily_activity
                WHERE activity_date >= date('now', 'localtime', '-6 days')
                  AND activity_date <= date('now', 'localtime')
                ",
                [],
                |row| row.get::<_, i64>(0),
            )
            .map_err(repository_error)?;

        let current_streak = self
            .connection
            .query_row(
                "
                WITH RECURSIVE
                streak_start(day) AS (
                    SELECT CASE
                        WHEN COALESCE((
                            SELECT answered_questions
                            FROM daily_activity
                            WHERE activity_date = date('now', 'localtime')
                        ), 0) >= ?1
                        THEN date('now', 'localtime')

                        WHEN COALESCE((
                            SELECT answered_questions
                            FROM daily_activity
                            WHERE activity_date = date('now', 'localtime', '-1 day')
                        ), 0) >= ?1
                        THEN date('now', 'localtime', '-1 day')

                        ELSE NULL
                    END
                ),
                streak(day, length) AS (
                    SELECT day, 1
                    FROM streak_start
                    WHERE day IS NOT NULL

                    UNION ALL

                    SELECT date(day, '-1 day'), length + 1
                    FROM streak
                    WHERE COALESCE((
                        SELECT answered_questions
                        FROM daily_activity
                        WHERE activity_date = date(streak.day, '-1 day')
                    ), 0) >= ?1
                )
                SELECT COALESCE(MAX(length), 0)
                FROM streak
                ",
                params![STUDY_DAY_QUESTION_TARGET as i64],
                |row| row.get::<_, i64>(0),
            )
            .map_err(repository_error)?;

        let best_streak = self
            .connection
            .query_row(
                "
                WITH study_days AS (
                    SELECT
                        activity_date,
                        ROW_NUMBER() OVER (ORDER BY activity_date) AS row_number
                    FROM daily_activity
                    WHERE answered_questions >= ?1
                ), grouped AS (
                    SELECT
                        activity_date,
                        julianday(activity_date) - row_number AS group_key
                    FROM study_days
                ), streaks AS (
                    SELECT COUNT(*) AS streak_length
                    FROM grouped
                    GROUP BY group_key
                )
                SELECT COALESCE(MAX(streak_length), 0)
                FROM streaks
                ",
                params![STUDY_DAY_QUESTION_TARGET as i64],
                |row| row.get::<_, i64>(0),
            )
            .map_err(repository_error)?;

        let recent_days = {
            let mut statement = self
                .connection
                .prepare(
                    "
                    WITH RECURSIVE days(day) AS (
                        SELECT date('now', 'localtime', '-27 days')

                        UNION ALL

                        SELECT date(day, '+1 day')
                        FROM days
                        WHERE day < date('now', 'localtime')
                    )
                    SELECT
                        days.day,
                        COALESCE(a.open_count, 0),
                        COALESCE(a.answered_questions, 0),
                        COALESCE(a.correct_answers, 0),
                        COALESCE(a.completed_sessions, 0)
                    FROM days
                    LEFT JOIN daily_activity a
                      ON a.activity_date = days.day
                    ORDER BY days.day
                    ",
                )
                .map_err(repository_error)?;

            let rows = statement
                .query_map([], |row| {
                    Ok(DailyActivity {
                        date: row.get(0)?,
                        open_count: row.get::<_, i64>(1)? as usize,
                        answered_questions: row.get::<_, i64>(2)? as usize,
                        correct_answers: row.get::<_, i64>(3)? as usize,
                        completed_sessions: row.get::<_, i64>(4)? as usize,
                    })
                })
                .map_err(repository_error)?;

            rows.collect::<rusqlite::Result<Vec<_>>>()
                .map_err(repository_error)?
        };

        let course_totals = {
            let mut statement = self
                .connection
                .prepare(
                    "
                    SELECT course_code, COUNT(*)
                    FROM topics
                    WHERE is_active = 1
                    GROUP BY course_code
                    ORDER BY course_code
                    ",
                )
                .map_err(repository_error)?;

            let rows = statement
                .query_map([], |row| {
                    Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
                })
                .map_err(repository_error)?;

            rows.collect::<rusqlite::Result<Vec<_>>>()
                .map_err(repository_error)?
        };

        let mut course_progress = Vec::with_capacity(course_totals.len());

        for (course_code, total_topics) in course_totals {
            let passed_topics = self
                .connection
                .query_row(
                    "
                    SELECT COUNT(DISTINCT topic_id)
                    FROM (
                        SELECT
                            s.id,
                            s.topic_id AS topic_id
                        FROM quiz_sessions s
                        JOIN attempts a
                          ON a.session_id = s.id
                        JOIN topics t
                          ON t.id = s.topic_id
                        WHERE s.scope = 'topic'
                          AND s.finished_at IS NOT NULL
                          AND s.cancelled_at IS NULL
                          AND s.is_review = 0
                          AND t.is_active = 1
                          AND t.course_code = ?1
                        GROUP BY s.id, s.topic_id
                        HAVING COUNT(a.id) > 0
                           AND (SUM(a.is_correct) * 100.0 / COUNT(a.id)) >= ?2
                    ) passed_sessions
                    ",
                    params![course_code, TOPIC_PASS_THRESHOLD_PERCENT],
                    |row| row.get::<_, i64>(0),
                )
                .map_err(repository_error)?;

            course_progress.push(CourseProgress {
                course_code,
                passed_topics: passed_topics as usize,
                total_topics: total_topics as usize,
            });
        }

        Ok(ActivityStats {
            active_days: active_days as usize,
            study_days: study_days as usize,
            current_streak: current_streak as usize,
            best_streak: best_streak as usize,
            total_answered_questions: total_answered_questions as usize,
            today_answered_questions: today_answered_questions as usize,
            today_correct_answers: today_correct_answers as usize,
            last_7_days_answered_questions: last_7_days_answered_questions as usize,
            recent_days,
            course_progress,
        })
    }

    fn record_app_open(&mut self) -> Result<(), RepositoryError> {
        self.connection
            .execute(
                "
                INSERT INTO daily_activity (
                    activity_date,
                    open_count
                )
                VALUES (
                    date('now', 'localtime'),
                    1
                )
                ON CONFLICT(activity_date) DO UPDATE SET
                    open_count = daily_activity.open_count + 1
                ",
                [],
            )
            .map_err(repository_error)?;

        Ok(())
    }
}

impl SessionRepository for SqliteRepository {
    fn start_session(&mut self, config: &SessionConfig) -> Result<QuizSession, RepositoryError> {
        let (scope, topic_id, course_code, is_review) = match &config.scope {
            QuizScope::Topic(topic_id) => ("topic", Some(topic_id.0), None, false),

            QuizScope::Course(course_code) => ("all", None, Some(course_code.as_str()), false),

            QuizScope::AllTopics => ("all", None, None, false),

            QuizScope::Mistakes => ("all", None, None, true),
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
                      AND (
                          ?3 = 0
                          OR EXISTS (
                              SELECT 1
                              FROM attempts previous_attempt
                              WHERE previous_attempt.question_id = q.id
                                AND previous_attempt.is_correct = 0
                          )
                      )
                    ORDER BY RANDOM()
                    LIMIT ?4
                    ",
                )
                .map_err(repository_error)?;

            let rows = statement
                .query_map(
                    params![topic_id, course_code, is_review as i64, sql_limit],
                    |row| row.get::<_, i64>(0),
                )
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
                    requested_count,
                    is_review
                )
                VALUES (?1, ?2, ?3, ?4, ?5)
                ",
                params![
                    scope,
                    topic_id,
                    course_code,
                    requested_count,
                    is_review as i64
                ],
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

        let completed_session = transaction
            .execute(
                "
            UPDATE quiz_sessions
            SET finished_at =
                CURRENT_TIMESTAMP
            WHERE id = ?1
              AND finished_at IS NULL
              AND current_index >= (
                  SELECT COUNT(*)
                  FROM session_questions
                  WHERE session_id = ?1
              )
            ",
                params![session_id.0],
            )
            .map_err(repository_error)?
            > 0;

        transaction
            .execute(
                "
                INSERT INTO daily_activity (
                    activity_date,
                    open_count,
                    answered_questions,
                    correct_answers,
                    completed_sessions
                )
                VALUES (
                    date('now', 'localtime'),
                    1,
                    1,
                    ?1,
                    ?2
                )
                ON CONFLICT(activity_date) DO UPDATE SET
                    open_count = MAX(daily_activity.open_count, 1),
                    answered_questions = daily_activity.answered_questions + 1,
                    correct_answers = daily_activity.correct_answers + excluded.correct_answers,
                    completed_sessions = daily_activity.completed_sessions + excluded.completed_sessions
                ",
                params![is_correct as i64, completed_session as i64],
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
                s.is_review,
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
                        row.get::<_, i64>(4)? != 0,
                        row.get::<_, i64>(5)?,
                        row.get::<_, i64>(6)?,
                        row.get::<_, i64>(7)?,
                        row.get::<_, i64>(8)?,
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
            is_review,
            current_index,
            total_questions,
            answered_questions,
            correct_answers,
        )) = row
        else {
            return Ok(None);
        };

        let scope = if is_review {
            QuizScope::Mistakes
        } else {
            match scope.as_str() {
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
                WHERE s.is_review = 0
                  AND (
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
                  AND s.is_review = 0
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
        let transaction = self.connection.transaction().map_err(repository_error)?;

        transaction
            .execute(
                "
                DELETE FROM quiz_sessions
                WHERE finished_at IS NOT NULL
                   OR cancelled_at IS NOT NULL
                ",
                [],
            )
            .map_err(repository_error)?;

        transaction
            .execute("DELETE FROM daily_activity", [])
            .map_err(repository_error)?;

        transaction.commit().map_err(repository_error)?;

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
    fn topic_mastery_tracks_best_result_and_attempt_to_pass() {
        let mut repository = seeded_repository();
        let config = SessionConfig {
            scope: QuizScope::Topic(TopicId(1)),
            limit: QuestionLimit::All,
        };

        let first = repository
            .start_session(&config)
            .expect("first session should start");
        let question = repository
            .current_question(first.id)
            .expect("question should load")
            .expect("question should exist");
        repository
            .submit_answer(first.id, question.id, &[AnswerOptionId(2)])
            .expect("wrong answer should still complete session");

        let second = repository
            .start_session(&config)
            .expect("second session should start");
        let question = repository
            .current_question(second.id)
            .expect("question should load")
            .expect("question should exist");
        repository
            .submit_answer(second.id, question.id, &[AnswerOptionId(1)])
            .expect("correct answer should complete session");

        let mastery = repository
            .topic_mastery(TopicId(1))
            .expect("mastery should load");

        assert_eq!(mastery.completed_attempts, 2);
        assert_eq!(mastery.best_accuracy_percent(), 100.0);
        assert_eq!(mastery.attempts_to_pass, Some(2));
        assert!(mastery.passed());
    }

    #[test]
    fn mistake_history_can_start_review_without_changing_main_statistics() {
        let mut repository = seeded_repository();
        let config = SessionConfig {
            scope: QuizScope::Topic(TopicId(1)),
            limit: QuestionLimit::All,
        };

        let session = repository
            .start_session(&config)
            .expect("topic session should start");
        let question = repository
            .current_question(session.id)
            .expect("question should load")
            .expect("question should exist");

        repository
            .submit_answer(session.id, question.id, &[AnswerOptionId(2)])
            .expect("wrong answer should be stored");

        let summaries = repository
            .mistake_summaries()
            .expect("mistake summaries should load");
        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].incorrect_attempts, 1);
        assert_eq!(summaries[0].total_attempts, 1);

        let detail = repository
            .mistake_detail(QuestionId(1))
            .expect("mistake detail should load");
        assert_eq!(detail.attempts.len(), 1);
        assert_eq!(
            detail.attempts[0].selected_answers,
            vec![String::from("Incorrect")]
        );
        assert_eq!(detail.correct_answers, vec![String::from("Correct")]);

        let review = repository
            .start_session(&SessionConfig {
                scope: QuizScope::Mistakes,
                limit: QuestionLimit::All,
            })
            .expect("mistake review should start");
        assert_eq!(review.question_ids, vec![QuestionId(1)]);

        repository
            .submit_answer(review.id, QuestionId(1), &[AnswerOptionId(1)])
            .expect("review answer should be accepted");

        let stats = repository
            .statistics(&StatisticsFilter::all())
            .expect("main statistics should load");
        assert_eq!(stats.completed_sessions, 1);
        assert_eq!(stats.answered_questions, 1);
        assert_eq!(stats.correct_answers, 0);

        let detail = repository
            .mistake_detail(QuestionId(1))
            .expect("updated mistake detail should load");
        assert_eq!(detail.attempts.len(), 2);
        assert_eq!(detail.summary.incorrect_attempts, 1);
        assert_eq!(detail.summary.total_attempts, 2);
    }

    #[test]
    fn activity_tracks_app_open_and_answered_questions() {
        let mut repository = seeded_repository();
        repository
            .record_app_open()
            .expect("app open should be recorded");

        let session = repository
            .start_session(&SessionConfig {
                scope: QuizScope::Topic(TopicId(1)),
                limit: QuestionLimit::All,
            })
            .expect("session should start");
        let question = repository
            .current_question(session.id)
            .expect("question should load")
            .expect("question should exist");
        repository
            .submit_answer(session.id, question.id, &[AnswerOptionId(1)])
            .expect("answer should be recorded");

        let activity = repository.activity_stats().expect("activity should load");

        assert_eq!(activity.active_days, 1);
        assert_eq!(activity.study_days, 0);
        assert_eq!(activity.today_answered_questions, 1);
        assert_eq!(activity.today_correct_answers, 1);
        assert_eq!(activity.total_answered_questions, 1);
        assert_eq!(activity.last_7_days_answered_questions, 1);
        assert_eq!(activity.recent_days.len(), 28);
        assert_eq!(activity.recent_days.last().unwrap().answered_questions, 1);
        assert_eq!(activity.passed_topics(), 1);
    }

    #[test]
    fn activity_streak_keeps_yesterdays_series_until_today_is_completed() {
        let repository = seeded_repository();

        repository
            .connection
            .execute_batch(
                "
                INSERT INTO daily_activity (
                    activity_date,
                    open_count,
                    answered_questions,
                    correct_answers,
                    completed_sessions
                )
                VALUES
                    (date('now', 'localtime', '-2 days'), 1, 10, 8, 1),
                    (date('now', 'localtime', '-1 day'), 1, 10, 9, 1),
                    (date('now', 'localtime'), 1, 0, 0, 0);
                ",
            )
            .expect("activity rows should be inserted");

        let activity = repository.activity_stats().expect("activity should load");

        assert_eq!(activity.study_days, 2);
        assert_eq!(activity.current_streak, 2);
        assert_eq!(activity.best_streak, 2);
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

        let mastery = repository
            .topic_mastery(TopicId(1))
            .expect("mastery should load after statistics reset");
        assert_eq!(mastery.completed_attempts, 0);
        assert_eq!(mastery.attempts_to_pass, None);

        let activity = repository
            .activity_stats()
            .expect("activity should load after statistics reset");
        assert_eq!(activity.active_days, 0);
        assert_eq!(activity.total_answered_questions, 0);

        let mistakes = repository
            .mistake_summaries()
            .expect("mistakes should load after statistics reset");
        assert!(mistakes.is_empty());

        let active = repository
            .active_session()
            .expect("active session should load")
            .expect("active session should remain");

        assert_eq!(active.id, active_session.id);
    }
}
