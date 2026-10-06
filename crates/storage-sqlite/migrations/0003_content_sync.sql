ALTER TABLE answer_options
    ADD COLUMN is_active INTEGER NOT NULL DEFAULT 1
        CHECK (is_active IN (0, 1));


DROP INDEX IF EXISTS ux_topics_content_key;

CREATE UNIQUE INDEX ux_topics_content_key
    ON topics (content_key);


DROP INDEX IF EXISTS ux_sources_content_key;

CREATE UNIQUE INDEX ux_sources_content_key
    ON sources (content_key);


DROP INDEX IF EXISTS ux_questions_content_key;

CREATE UNIQUE INDEX ux_questions_content_key
    ON questions (content_key);


DROP INDEX IF EXISTS ux_answer_options_content_key;

CREATE UNIQUE INDEX ux_answer_options_question_content_key
    ON answer_options (
                       question_id,
                       content_key
        );