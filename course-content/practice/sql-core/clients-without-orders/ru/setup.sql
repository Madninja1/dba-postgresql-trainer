CREATE TABLE customers (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE orders (
    id INTEGER PRIMARY KEY,
    customer_id INTEGER NOT NULL REFERENCES customers(id),
    amount INTEGER NOT NULL
);

INSERT INTO customers (id, name) VALUES
    (1, 'Ada'),
    (2, 'Linus'),
    (3, 'Grace'),
    (4, 'Ken');

INSERT INTO orders (id, customer_id, amount) VALUES
    (1, 1, 120),
    (2, 1, 75),
    (3, 3, 200),
    (4, 4, 50);
