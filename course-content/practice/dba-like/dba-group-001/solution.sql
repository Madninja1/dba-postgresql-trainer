SELECT database_name, COUNT(*) AS active_sessions
FROM sessions
WHERE state = 'active'
GROUP BY database_name
HAVING COUNT(*) >= 2
ORDER BY active_sessions DESC, database_name ASC;
