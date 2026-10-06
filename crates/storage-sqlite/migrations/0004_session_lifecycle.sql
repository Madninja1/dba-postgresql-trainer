ALTER TABLE quiz_sessions
    ADD COLUMN cancelled_at TEXT;

CREATE INDEX idx_quiz_sessions_state
    ON quiz_sessions (
                      finished_at,
                      cancelled_at,
                      id
        );