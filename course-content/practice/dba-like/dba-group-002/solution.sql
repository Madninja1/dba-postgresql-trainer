SELECT database_name, ROUND(AVG(duration_ms), 1) AS avg_duration_ms
FROM query_stats
GROUP BY database_name
HAVING COUNT(*) >= 3
   AND AVG(duration_ms) >= 200
ORDER BY avg_duration_ms DESC, database_name ASC;
