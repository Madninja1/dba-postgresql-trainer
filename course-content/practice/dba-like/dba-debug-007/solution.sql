SELECT database_name, ROUND(AVG(duration_sec), 1) AS avg_duration_sec
       FROM sessions
       WHERE state = 'active'
       GROUP BY database_name
       ORDER BY avg_duration_sec DESC, database_name ASC;
