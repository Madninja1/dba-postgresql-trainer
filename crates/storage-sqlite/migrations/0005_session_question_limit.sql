ALTER TABLE quiz_sessions
    ADD COLUMN question_limit TEXT NOT NULL DEFAULT 'unknown'
        CHECK (
            question_limit IN (
                               '20',
                               '50',
                               'all',
                               'unknown'
                )
            );

CREATE INDEX idx_quiz_sessions_question_limit
    ON quiz_sessions (question_limit);