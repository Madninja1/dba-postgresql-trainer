CREATE TABLE practice_tasks (
    id INTEGER PRIMARY KEY,
    content_key TEXT NOT NULL UNIQUE,
    category TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    practice_type TEXT NOT NULL,
    comparison TEXT NOT NULL,
    starter_sql TEXT NOT NULL DEFAULT '',
    setup_sql TEXT NOT NULL,
    solution_sql TEXT NOT NULL,
    sort_order INTEGER NOT NULL DEFAULT 0,
    is_active INTEGER NOT NULL DEFAULT 1
);

CREATE INDEX idx_practice_tasks_category
    ON practice_tasks(category, sort_order, id);
