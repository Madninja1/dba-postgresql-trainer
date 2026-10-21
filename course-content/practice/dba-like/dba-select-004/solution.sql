SELECT database_name, size_gb
       FROM database_health
       WHERE environment = 'production' AND accepting_connections = 0
       ORDER BY size_gb DESC, database_name ASC;
