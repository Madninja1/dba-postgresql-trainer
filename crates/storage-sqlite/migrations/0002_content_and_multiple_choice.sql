ALTER TABLE topics
    ADD COLUMN course_code TEXT NOT NULL
        DEFAULT 'dba-1';

ALTER TABLE topics
    ADD COLUMN content_key TEXT;

CREATE UNIQUE INDEX ux_topics_content_key
    ON topics (content_key)
    WHERE content_key IS NOT NULL;


ALTER TABLE sources
    ADD COLUMN kind TEXT NOT NULL
        DEFAULT 'course_material'
        CHECK (
            kind IN (
                     'course_material',
                     'postgresql_docs'
                )
            );

ALTER TABLE sources
    ADD COLUMN url TEXT;

ALTER TABLE sources
    ADD COLUMN content_key TEXT;

CREATE UNIQUE INDEX ux_sources_content_key
    ON sources (content_key)
    WHERE content_key IS NOT NULL;


ALTER TABLE questions
    ADD COLUMN question_type TEXT NOT NULL
        DEFAULT 'single_choice'
        CHECK (
            question_type IN (
                              'single_choice',
                              'multiple_choice'
                )
            );

ALTER TABLE questions
    ADD COLUMN content_key TEXT;

CREATE UNIQUE INDEX ux_questions_content_key
    ON questions (content_key)
    WHERE content_key IS NOT NULL;


ALTER TABLE answer_options
    ADD COLUMN content_key TEXT;

CREATE UNIQUE INDEX ux_answer_options_content_key
    ON answer_options (content_key)
    WHERE content_key IS NOT NULL;


CREATE TABLE _migration_attempt_answers
(
    attempt_id INTEGER NOT NULL,
    question_id INTEGER NOT NULL,
    answer_option_id INTEGER NOT NULL
);

INSERT INTO
    _migration_attempt_answers (attempt_id,
                                question_id,
                                answer_option_id)
SELECT
    rowid,
    question_id,
    selected_option_id
FROM
    attempts;


CREATE TABLE attempts_v2
(
    id INTEGER PRIMARY KEY,

    session_id INTEGER NOT NULL,
    question_id INTEGER NOT NULL,

    is_correct INTEGER NOT NULL
        CHECK (is_correct IN (0, 1)),

    answered_at TEXT NOT NULL
        DEFAULT CURRENT_TIMESTAMP,

    UNIQUE (session_id, question_id),
    UNIQUE (id, question_id),

    FOREIGN KEY (session_id, question_id)
        REFERENCES session_questions (
                                      session_id,
                                      question_id
            )
        ON DELETE CASCADE
);

INSERT INTO
    attempts_v2 (id,
                 session_id,
                 question_id,
                 is_correct,
                 answered_at)
SELECT
    rowid,
    session_id,
    question_id,
    is_correct,
    answered_at
FROM
    attempts;

DROP TABLE attempts;

ALTER TABLE attempts_v2
    RENAME TO attempts;


CREATE TABLE attempt_answers
(
    attempt_id INTEGER NOT NULL,
    question_id INTEGER NOT NULL,
    answer_option_id INTEGER NOT NULL,

    PRIMARY KEY (
                 attempt_id,
                 answer_option_id
        ),

    FOREIGN KEY (
                 attempt_id,
                 question_id
        )
        REFERENCES attempts (
                             id,
                             question_id
            )
        ON DELETE CASCADE,

    FOREIGN KEY (
                 answer_option_id,
                 question_id
        )
        REFERENCES answer_options (
                                   id,
                                   question_id
            )
);

INSERT INTO
    attempt_answers (attempt_id,
                     question_id,
                     answer_option_id)
SELECT
    attempt_id,
    question_id,
    answer_option_id
FROM
    _migration_attempt_answers;

DROP TABLE _migration_attempt_answers;


CREATE INDEX idx_attempts_question
    ON attempts (question_id);

CREATE INDEX idx_attempt_answers_attempt
    ON attempt_answers (attempt_id);