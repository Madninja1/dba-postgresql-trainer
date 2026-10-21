SELECT state, COUNT(*) AS session_count
       FROM sessions
       GROUP BY state
       HAVING COUNT(*) >= 2
       ORDER BY session_count DESC, state ASC;
