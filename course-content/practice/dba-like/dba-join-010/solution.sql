SELECT i.id AS incident_id, s.hostname, i.severity, i.opened_seq
       FROM incidents AS i
       JOIN servers AS s ON s.id = i.server_id
       WHERE i.resolved = 0 AND s.environment = 'production'
       ORDER BY i.opened_seq ASC, i.id ASC;
