SELECT s.id AS session_id, s.database_name, s.duration_sec
       FROM sessions AS s
       WHERE s.state = 'active'
         AND s.duration_sec > (
           SELECT AVG(s2.duration_sec)
           FROM sessions AS s2
           WHERE s2.database_name = s.database_name AND s2.state = 'active'
         )
       ORDER BY s.database_name ASC, s.duration_sec DESC, s.id ASC;
