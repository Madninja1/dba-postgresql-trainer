SELECT server_name,
              SUM(CASE WHEN resolved = 0 THEN 1 ELSE 0 END) AS open_incidents,
              SUM(CASE WHEN resolved = 0 AND severity = 'critical' THEN 1 ELSE 0 END) AS critical_open
       FROM incidents
       GROUP BY server_name
       HAVING SUM(CASE WHEN resolved = 0 AND severity = 'critical' THEN 1 ELSE 0 END) >= 1
       ORDER BY critical_open DESC, open_incidents DESC, server_name ASC;
