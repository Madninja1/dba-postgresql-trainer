SELECT database_name,
              SUM(CASE WHEN state = 'active' THEN 1 ELSE 0 END) AS active_sessions,
              SUM(CASE WHEN state = 'idle' THEN 1 ELSE 0 END) AS idle_sessions
       FROM sessions
       GROUP BY database_name
       HAVING SUM(CASE WHEN state = 'active' THEN 1 ELSE 0 END) >= 1
       ORDER BY active_sessions DESC, idle_sessions DESC, database_name ASC;
