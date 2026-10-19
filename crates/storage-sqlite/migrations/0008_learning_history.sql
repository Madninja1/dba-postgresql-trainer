ALTER TABLE quiz_sessions
    ADD COLUMN is_review INTEGER NOT NULL DEFAULT 0
        CHECK (is_review IN (0, 1));

CREATE INDEX idx_quiz_sessions_review
    ON quiz_sessions (is_review, finished_at, cancelled_at, id);

CREATE INDEX idx_attempts_answered_at
    ON attempts (answered_at);

CREATE TABLE daily_activity
(
    activity_date TEXT PRIMARY KEY,
    open_count INTEGER NOT NULL DEFAULT 0
        CHECK (open_count >= 0),
    answered_questions INTEGER NOT NULL DEFAULT 0
        CHECK (answered_questions >= 0),
    correct_answers INTEGER NOT NULL DEFAULT 0
        CHECK (correct_answers >= 0),
    completed_sessions INTEGER NOT NULL DEFAULT 0
        CHECK (completed_sessions >= 0)
);

INSERT INTO daily_activity (
    activity_date,
    open_count,
    answered_questions,
    correct_answers,
    completed_sessions
)
SELECT
    activity_date,
    1,
    SUM(answered_questions),
    SUM(correct_answers),
    SUM(completed_sessions)
FROM (
    SELECT
        date(started_at, 'localtime') AS activity_date,
        0 AS answered_questions,
        0 AS correct_answers,
        0 AS completed_sessions
    FROM quiz_sessions

    UNION ALL

    SELECT
        date(answered_at, 'localtime') AS activity_date,
        COUNT(*) AS answered_questions,
        COALESCE(SUM(is_correct), 0) AS correct_answers,
        0 AS completed_sessions
    FROM attempts
    GROUP BY date(answered_at, 'localtime')

    UNION ALL

    SELECT
        date(finished_at, 'localtime') AS activity_date,
        0 AS answered_questions,
        0 AS correct_answers,
        COUNT(*) AS completed_sessions
    FROM quiz_sessions
    WHERE finished_at IS NOT NULL
      AND cancelled_at IS NULL
    GROUP BY date(finished_at, 'localtime')
)
WHERE activity_date IS NOT NULL
GROUP BY activity_date;
