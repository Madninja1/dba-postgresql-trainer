SELECT session_id, environment, state
       FROM sessions
       WHERE environment = 'production'
         AND (state = 'active' OR state = 'idle in transaction')
       ORDER BY session_id ASC;
