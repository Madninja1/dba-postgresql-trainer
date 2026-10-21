SELECT database_name, COUNT(*) AS backup_count,
              ROUND(100.0 * SUM(CASE WHEN status = 'success' THEN 1 ELSE 0 END) / COUNT(*), 1) AS success_pct
       FROM backups
       GROUP BY database_name
       HAVING COUNT(*) >= 3
          AND 100.0 * SUM(CASE WHEN status = 'success' THEN 1 ELSE 0 END) / COUNT(*) < 80.0
       ORDER BY success_pct ASC, database_name ASC;
