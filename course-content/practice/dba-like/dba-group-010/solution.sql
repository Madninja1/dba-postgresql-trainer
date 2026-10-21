SELECT database_name,
              COUNT(*) AS sample_count,
              ROUND(AVG(duration_ms), 1) AS avg_duration_ms,
              MAX(duration_ms) AS max_duration_ms,
              SUM(CASE WHEN duration_ms >= 500 THEN 1 ELSE 0 END) AS slow_count
       FROM query_stats
       GROUP BY database_name
       HAVING COUNT(*) >= 4
          AND SUM(CASE WHEN duration_ms >= 500 THEN 1 ELSE 0 END) >= 2
          AND AVG(duration_ms) >= 200
       ORDER BY slow_count DESC, avg_duration_ms DESC, database_name ASC;
