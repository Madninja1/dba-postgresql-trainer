WITH active AS (
         SELECT database_id, COUNT(*) AS active_sessions
         FROM sessions
         WHERE state = 'active'
         GROUP BY database_id
       ),
       failed AS (
         SELECT database_id, COUNT(*) AS failed_backups
         FROM backups
         WHERE status = 'failed'
         GROUP BY database_id
       )
       SELECT d.name AS database_name,
              COALESCE(a.active_sessions, 0) AS active_sessions,
              COALESCE(f.failed_backups, 0) AS failed_backups,
              COALESCE(a.active_sessions, 0) * 2 + COALESCE(f.failed_backups, 0) * 3 AS risk_score
       FROM databases AS d
       LEFT JOIN active AS a ON a.database_id = d.id
       LEFT JOIN failed AS f ON f.database_id = d.id
       WHERE COALESCE(a.active_sessions, 0) * 2 + COALESCE(f.failed_backups, 0) * 3 >= 5
       ORDER BY risk_score DESC, d.name ASC;
