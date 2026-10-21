WITH session_counts AS (
    SELECT srv.id, srv.hostname, COUNT(s.id) AS active_sessions
    FROM servers AS srv
    LEFT JOIN sessions AS s
      ON s.server_id = srv.id
     AND s.state = 'active'
    GROUP BY srv.id, srv.hostname
)
SELECT hostname, active_sessions
FROM session_counts
WHERE active_sessions > (SELECT AVG(active_sessions) FROM session_counts)
ORDER BY active_sessions DESC, hostname ASC;
