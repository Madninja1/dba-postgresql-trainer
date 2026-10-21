SELECT session_id, database_name, started_seq,
              ROW_NUMBER() OVER (ORDER BY started_seq ASC, session_id ASC) AS age_rank
       FROM sessions
       WHERE state = 'active'
       ORDER BY age_rank ASC;
