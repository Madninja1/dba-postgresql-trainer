ALTER TABLE practice_tasks ADD COLUMN difficulty TEXT NOT NULL DEFAULT 'medium';
ALTER TABLE practice_tasks ADD COLUMN solution_explanation TEXT;

CREATE TABLE practice_hints (
    task_id INTEGER NOT NULL,
    position INTEGER NOT NULL,
    text TEXT NOT NULL,
    PRIMARY KEY (task_id, position),
    FOREIGN KEY (task_id) REFERENCES practice_tasks(id) ON DELETE CASCADE
);

CREATE TABLE practice_rules (
    task_id INTEGER NOT NULL,
    position INTEGER NOT NULL,
    rule_type TEXT NOT NULL,
    value TEXT,
    PRIMARY KEY (task_id, position),
    FOREIGN KEY (task_id) REFERENCES practice_tasks(id) ON DELETE CASCADE
);

CREATE TABLE practice_assertions (
    task_id INTEGER NOT NULL,
    position INTEGER NOT NULL,
    assertion_type TEXT NOT NULL,
    name TEXT,
    sql TEXT,
    expected_text TEXT,
    expected_int INTEGER,
    PRIMARY KEY (task_id, position),
    FOREIGN KEY (task_id) REFERENCES practice_tasks(id) ON DELETE CASCADE
);

CREATE TABLE practice_progress (
    task_id INTEGER PRIMARY KEY,
    last_sql TEXT NOT NULL DEFAULT '',
    attempts INTEGER NOT NULL DEFAULT 0,
    solved INTEGER NOT NULL DEFAULT 0,
    solved_attempt INTEGER,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (task_id) REFERENCES practice_tasks(id) ON DELETE CASCADE
);

CREATE TABLE practice_attempts (
    id INTEGER PRIMARY KEY,
    task_id INTEGER NOT NULL,
    submitted_sql TEXT NOT NULL,
    success INTEGER NOT NULL,
    attempted_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (task_id) REFERENCES practice_tasks(id) ON DELETE CASCADE
);

CREATE INDEX idx_practice_progress_updated ON practice_progress(updated_at DESC);
CREATE INDEX idx_practice_attempts_task ON practice_attempts(task_id, attempted_at DESC);
