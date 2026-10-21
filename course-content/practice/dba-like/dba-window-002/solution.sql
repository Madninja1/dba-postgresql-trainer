WITH history AS (
    SELECT database_name,
           id AS backup_id,
           size_mb,
           LAG(size_mb) OVER (
               PARTITION BY database_name
               ORDER BY finished_seq, id
           ) AS previous_size_mb
    FROM backups
    WHERE status = 'success'
)
SELECT database_name,
       backup_id,
       size_mb,
       previous_size_mb,
       size_mb - previous_size_mb AS growth_mb
FROM history
WHERE previous_size_mb IS NOT NULL
  AND size_mb - previous_size_mb >= 100
ORDER BY database_name ASC, backup_id ASC;
