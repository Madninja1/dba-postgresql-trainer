WITH ranked AS (
    SELECT database_name,
           id AS query_id,
           duration_ms,
           ROW_NUMBER() OVER (
               PARTITION BY database_name
               ORDER BY duration_ms DESC, id ASC
           ) AS rn
    FROM query_stats
)
SELECT database_name, query_id, duration_ms
FROM ranked
WHERE rn <= 2
ORDER BY database_name ASC, rn ASC;
