# DBA-like SQL practice

The v1 bank contains 60 PostgreSQL-DBA-themed tasks executed locally by SQLite.
Each task keeps localized `practice.json` files, one shared `solution.sql`, and three datasets:
`cases/visible.sql`, `cases/hidden-01.sql`, and `cases/hidden-02.sql`.

The visible case powers the workbench. `Check` also runs both hidden cases, so a query must solve the problem rather than match one particular dataset.

Final v1 bank: 60 tasks — 10 SELECT/WHERE/ORDER BY, 10 GROUP BY/HAVING, 15 JOIN, 8 Subquery/CTE, 8 Window functions, and 9 FixQuery/FindLogicalError. Difficulty distribution: 20 Easy, 28 Medium, 12 Hard.

## Task wording requirements

A task description is the result contract, not an implementation hint. Every task must explicitly state the exact output columns and their order, all filtering conditions, edge-case behavior, and the complete sort order including direction and tie-breakers. If a specific technique (`CTE`, `JOIN`, `ROW_NUMBER()`, etc.) is required, state it as a task constraint. Hints may explain an approach only after the learner asks for them and must not contain requirements missing from the main description.
