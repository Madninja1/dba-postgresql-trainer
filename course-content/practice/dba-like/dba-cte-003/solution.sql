WITH active_counts AS (
         SELECT database_name, COUNT(*) AS active_sessions
         FROM sessions
         WHERE state = 'active'
         GROUP BY database_name
       )
       SELECT database_name, active_sessions
       FROM active_counts
       WHERE active_sessions >= 2
       ORDER BY active_sessions DESC, database_name ASC;
