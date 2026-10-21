ALTER TABLE practice_progress ADD COLUMN hints_revealed INTEGER NOT NULL DEFAULT 0;

CREATE TABLE practice_hidden_cases (
    task_id INTEGER NOT NULL,
    position INTEGER NOT NULL,
    setup_sql TEXT NOT NULL,
    PRIMARY KEY (task_id, position),
    FOREIGN KEY (task_id) REFERENCES practice_tasks(id) ON DELETE CASCADE
);
