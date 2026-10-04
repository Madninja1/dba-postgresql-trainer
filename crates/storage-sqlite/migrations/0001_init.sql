CREATE TABLE topics
(
    id INTEGER PRIMARY KEY,
    slug TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    description TEXT,
    sort_order INTEGER NOT NULL DEFAULT 0,
    is_active INTEGER NOT NULL DEFAULT 1
        CHECK (is_active IN (0, 1))
);

CREATE TABLE sources
(
    id INTEGER PRIMARY KEY,
    module TEXT NOT NULL,
    section TEXT NOT NULL,
    locator TEXT NOT NULL
);

CREATE TABLE questions
(
    id INTEGER PRIMARY KEY,
    topic_id INTEGER NOT NULL,
    source_id INTEGER NOT NULL,

    text TEXT NOT NULL,
    explanation TEXT NOT NULL,

    is_active INTEGER NOT NULL DEFAULT 1
        CHECK (is_active IN (0, 1)),

    FOREIGN KEY (topic_id)
        REFERENCES topics (id),

    FOREIGN KEY (source_id)
        REFERENCES sources (id)
);

CREATE TABLE answer_options
(
    id INTEGER PRIMARY KEY,
    question_id INTEGER NOT NULL,

    text TEXT NOT NULL,
    is_correct INTEGER NOT NULL,

    sort_order INTEGER NOT NULL DEFAULT 0,

    UNIQUE (id, question_id),

    FOREIGN KEY (question_id)
        REFERENCES questions (id)
        ON DELETE CASCADE
);

CREATE TABLE quiz_sessions
(
    id INTEGER PRIMARY KEY,
    scope TEXT NOT NULL
        CHECK (scope IN ('topic', 'all')),

    topic_id INTEGER,

    requested_count INTEGER
        CHECK (
            requested_count IS NULL
                    OR requested_count IN (20, 50)
            ),

    current_index INTEGER NOT NULL DEFAULT 0
        CHECK (current_index >= 0),

    started_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    finished_at TEXT,
    CHECK (
        (scope = 'topic' AND topic_id IS NOT NULL)
                OR
            (scope = 'all' AND topic_id IS NULL)
        ),

    FOREIGN KEY (topic_id)
        REFERENCES topics (id)
);

CREATE TABLE session_questions
(
    session_id INTEGER NOT NULL,
    question_id INTEGER NOT NULL,
    position INTEGER NOT NULL,

    PRIMARY KEY (session_id, question_id),

    UNIQUE (session_id, position),

    FOREIGN KEY (session_id)
        REFERENCES quiz_sessions (id)
        ON DELETE CASCADE,

    FOREIGN KEY (question_id)
        REFERENCES questions (id)
);

CREATE TABLE attempts
(
    session_id INTEGER NOT NULL,
    question_id INTEGER NOT NULL,
    selected_option_id INTEGER NOT NULL,

    is_correct INTEGER NOT NULL
        CHECK (is_correct IN (0, 1)),

    answered_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (session_id, question_id),

    FOREIGN KEY (session_id, question_id)
        REFERENCES session_questions (session_id, question_id)
        ON DELETE CASCADE,

    FOREIGN KEY (selected_option_id, question_id)
        REFERENCES answer_options (id, question_id)
);

CREATE INDEX idx_questions_topic
    ON questions(topic_id, is_active);

CREATE INDEX idx_answer_options_question
    ON answer_options(question_id, sort_order);