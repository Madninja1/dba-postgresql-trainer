SELECT d.name AS database_name, COUNT(s.id) AS session_count
       FROM databases AS d
       LEFT JOIN sessions AS s ON s.database_id = d.id
       GROUP BY d.id, d.name
       ORDER BY session_count DESC, d.name ASC;
