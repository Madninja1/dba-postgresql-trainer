ALTER TABLE quiz_sessions
    ADD COLUMN course_code TEXT;

CREATE INDEX idx_quiz_sessions_course_code
    ON quiz_sessions (course_code);
