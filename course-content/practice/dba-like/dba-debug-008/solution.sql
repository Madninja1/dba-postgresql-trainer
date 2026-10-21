WITH failed AS (
         SELECT d.server_id, COUNT(*) AS failed_backups
         FROM databases AS d
         JOIN backups AS b ON b.database_id = d.id
         WHERE b.status = 'failed'
         GROUP BY d.server_id
       ),
       opened AS (
         SELECT server_id, COUNT(*) AS open_incidents
         FROM incidents
         WHERE resolved = 0
         GROUP BY server_id
       )
       SELECT s.hostname AS server_name,
              COALESCE(f.failed_backups, 0) AS failed_backups,
              COALESCE(o.open_incidents, 0) AS open_incidents
       FROM servers AS s
       LEFT JOIN failed AS f ON f.server_id = s.id
       LEFT JOIN opened AS o ON o.server_id = s.id
       ORDER BY s.hostname ASC;
