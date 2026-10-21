SELECT replica_name, database_name, lag_seconds
       FROM replicas
       WHERE state = 'streaming' AND lag_seconds >= 30
       ORDER BY lag_seconds DESC, replica_name ASC;
