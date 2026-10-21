SELECT session_id, database_name, duration_sec, wait_event
       FROM sessions
       WHERE state = 'active' AND duration_sec >= 60 AND wait_event IS NOT NULL
       ORDER BY duration_sec DESC, session_id ASC;
