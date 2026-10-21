ALTER TABLE daily_activity
    ADD COLUMN practice_attempts INTEGER NOT NULL DEFAULT 0
        CHECK (practice_attempts >= 0);

ALTER TABLE daily_activity
    ADD COLUMN practice_solved INTEGER NOT NULL DEFAULT 0
        CHECK (practice_solved >= 0);

CREATE TABLE xp_events (
    id INTEGER PRIMARY KEY,
    event_key TEXT NOT NULL UNIQUE,
    points INTEGER NOT NULL CHECK (points > 0),
    reason TEXT NOT NULL,
    earned_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_xp_events_earned_at
    ON xp_events (earned_at DESC);

-- Backfill one point for every question that has ever been answered correctly.
INSERT OR IGNORE INTO xp_events (event_key, points, reason, earned_at)
SELECT
    'question:correct:' || a.question_id,
    1,
    'question_correct',
    MIN(a.answered_at)
FROM attempts a
WHERE a.is_correct = 1
GROUP BY a.question_id;

-- Backfill passed non-review quiz bonuses. The same material/mode only grants this bonus once.
WITH passed_sessions AS (
    SELECT
        s.id,
        CASE
            WHEN s.scope = 'topic' AND s.topic_id IS NOT NULL
                THEN 'topic:' || s.topic_id
            WHEN s.course_code IS NOT NULL
                THEN 'course:' || s.course_code
            ELSE 'all'
        END AS material_key,
        CASE
            WHEN s.requested_count IS NULL THEN 'all'
            ELSE CAST(s.requested_count AS TEXT)
        END AS limit_key,
        CASE
            WHEN s.requested_count = 20 THEN 10
            WHEN s.requested_count = 50 THEN 20
            WHEN s.requested_count IS NULL THEN 30
            ELSE 10
        END AS points,
        s.finished_at
    FROM quiz_sessions s
    JOIN attempts a ON a.session_id = s.id
    WHERE s.finished_at IS NOT NULL
      AND s.cancelled_at IS NULL
      AND s.is_review = 0
    GROUP BY
        s.id,
        s.scope,
        s.topic_id,
        s.course_code,
        s.requested_count,
        s.finished_at
    HAVING COUNT(a.id) > 0
       AND (SUM(a.is_correct) * 100.0 / COUNT(a.id)) >= CASE
           WHEN s.requested_count = 20 THEN 85.0
           WHEN s.requested_count = 50 THEN 70.0
           WHEN s.requested_count IS NULL THEN 60.0
           ELSE 85.0
       END
)
INSERT OR IGNORE INTO xp_events (event_key, points, reason, earned_at)
SELECT
    'quiz:passed:' || material_key || ':' || limit_key,
    MAX(points),
    'quiz_passed',
    MIN(finished_at)
FROM passed_sessions
GROUP BY material_key, limit_key;

-- Backfill the first mastered-topic bonus. A topic is mastered by a passed topic quiz.
INSERT OR IGNORE INTO xp_events (event_key, points, reason, earned_at)
SELECT
    'topic:mastered:' || passed.topic_id,
    50,
    'topic_mastered',
    MIN(passed.finished_at)
FROM (
    SELECT
        s.id,
        s.topic_id,
        s.finished_at,
        s.requested_count,
        COUNT(a.id) AS answered,
        SUM(a.is_correct) AS correct
    FROM quiz_sessions s
    JOIN attempts a ON a.session_id = s.id
    WHERE s.scope = 'topic'
      AND s.topic_id IS NOT NULL
      AND s.finished_at IS NOT NULL
      AND s.cancelled_at IS NULL
      AND s.is_review = 0
    GROUP BY s.id, s.topic_id, s.finished_at, s.requested_count
) passed
WHERE passed.answered > 0
  AND (passed.correct * 100.0 / passed.answered) >= CASE
      WHEN passed.requested_count = 20 THEN 85.0
      WHEN passed.requested_count = 50 THEN 70.0
      WHEN passed.requested_count IS NULL THEN 60.0
      ELSE 85.0
  END
GROUP BY passed.topic_id;

-- Backfill completed-course bonuses only for courses that are complete at migration time.
INSERT OR IGNORE INTO xp_events (event_key, points, reason)
SELECT
    'course:completed:' || t.course_code,
    200,
    'course_completed'
FROM topics t
WHERE t.is_active = 1
GROUP BY t.course_code
HAVING COUNT(*) > 0
   AND COUNT(*) = SUM(
       CASE WHEN EXISTS (
           SELECT 1
           FROM quiz_sessions s
           JOIN attempts a ON a.session_id = s.id
           WHERE s.scope = 'topic'
             AND s.topic_id = t.id
             AND s.finished_at IS NOT NULL
             AND s.cancelled_at IS NULL
             AND s.is_review = 0
           GROUP BY s.id, s.requested_count
           HAVING COUNT(a.id) > 0
              AND (SUM(a.is_correct) * 100.0 / COUNT(a.id)) >= CASE
                  WHEN s.requested_count = 20 THEN 85.0
                  WHEN s.requested_count = 50 THEN 70.0
                  WHEN s.requested_count IS NULL THEN 60.0
                  ELSE 85.0
              END
       ) THEN 1 ELSE 0 END
   );

-- Backfill SQL practice XP. Only the first successful attempt of a task grants task XP.
INSERT OR IGNORE INTO xp_events (event_key, points, reason, earned_at)
SELECT
    'practice:solved:' || p.task_id,
    CASE t.difficulty
        WHEN 'easy' THEN 10
        WHEN 'hard' THEN 35
        ELSE 20
    END,
    'practice_solved',
    first_attempt.attempted_at
FROM practice_progress p
JOIN practice_tasks t ON t.id = p.task_id
JOIN (
    SELECT pa.task_id, pa.attempted_at
    FROM practice_attempts pa
    JOIN (
        SELECT task_id, MIN(id) AS first_attempt_id
        FROM practice_attempts
        WHERE success = 1
        GROUP BY task_id
    ) first_solve ON first_solve.first_attempt_id = pa.id
) first_attempt ON first_attempt.task_id = p.task_id
WHERE p.solved = 1;

INSERT OR IGNORE INTO xp_events (event_key, points, reason, earned_at)
SELECT
    'practice:first_try:' || p.task_id,
    5,
    'practice_first_try',
    first_attempt.attempted_at
FROM practice_progress p
JOIN (
    SELECT pa.task_id, pa.attempted_at
    FROM practice_attempts pa
    JOIN (
        SELECT task_id, MIN(id) AS first_attempt_id
        FROM practice_attempts
        WHERE success = 1
        GROUP BY task_id
    ) first_solve ON first_solve.first_attempt_id = pa.id
) first_attempt ON first_attempt.task_id = p.task_id
WHERE p.solved = 1
  AND p.solved_attempt = 1;

-- Existing practice attempts become part of daily learning activity.
INSERT INTO daily_activity (
    activity_date,
    open_count,
    practice_attempts,
    practice_solved
)
SELECT
    date(pa.attempted_at, 'localtime'),
    1,
    COUNT(*),
    SUM(CASE
        WHEN pa.id = first_solve.first_attempt_id THEN 1
        ELSE 0
    END)
FROM practice_attempts pa
LEFT JOIN (
    SELECT task_id, MIN(id) AS first_attempt_id
    FROM practice_attempts
    WHERE success = 1
    GROUP BY task_id
) first_solve ON first_solve.task_id = pa.task_id
GROUP BY date(pa.attempted_at, 'localtime')
ON CONFLICT(activity_date) DO UPDATE SET
    open_count = MAX(daily_activity.open_count, excluded.open_count),
    practice_attempts = daily_activity.practice_attempts + excluded.practice_attempts,
    practice_solved = daily_activity.practice_solved + excluded.practice_solved;
