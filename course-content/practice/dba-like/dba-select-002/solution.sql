SELECT server_name, name, size_mb, connection_usage_percent
FROM databases
WHERE status = 'online'
  AND (size_mb >= 51200 OR connection_usage_percent >= 80)
ORDER BY connection_usage_percent DESC, name ASC;
