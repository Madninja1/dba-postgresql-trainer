SELECT server_name, COUNT(*) AS failed_backups
FROM backups
WHERE status = 'failed'
  AND age_days <= 7
GROUP BY server_name
HAVING COUNT(*) >= 2
ORDER BY failed_backups DESC, server_name ASC;
