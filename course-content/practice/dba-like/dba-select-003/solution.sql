SELECT id, server_name, severity, age_hours
FROM incidents
WHERE resolved_at IS NULL
  AND severity IN ('critical', 'high')
ORDER BY CASE severity WHEN 'critical' THEN 0 ELSE 1 END,
         age_hours DESC,
         id ASC;
