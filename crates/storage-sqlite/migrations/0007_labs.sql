CREATE TABLE labs
(
    id INTEGER PRIMARY KEY,
    course_code TEXT NOT NULL,
    content_key TEXT NOT NULL UNIQUE,
    lab_number INTEGER NOT NULL
        CHECK (lab_number > 0),
    title TEXT NOT NULL,
    description TEXT,
    body TEXT NOT NULL,
    sort_order INTEGER NOT NULL DEFAULT 0,
    is_active INTEGER NOT NULL DEFAULT 1
        CHECK (is_active IN (0, 1))
);

CREATE INDEX idx_labs_course_number
    ON labs (course_code, lab_number, sort_order);
