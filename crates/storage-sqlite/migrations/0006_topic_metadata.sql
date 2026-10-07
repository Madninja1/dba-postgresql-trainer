ALTER TABLE topics
    ADD COLUMN notes_part INTEGER NOT NULL DEFAULT 1
        CHECK (notes_part > 0);

ALTER TABLE topics
    ADD COLUMN topic_number INTEGER NOT NULL DEFAULT 1
        CHECK (topic_number > 0);

CREATE INDEX idx_topics_course_topic_number
    ON topics (course_code, topic_number, sort_order);
