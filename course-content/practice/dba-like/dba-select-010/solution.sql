SELECT database_name, free_percent, connections_used, connections_limit, risk_reason
       FROM (
         SELECT database_name, free_percent, connections_used, connections_limit,
                CASE
                  WHEN read_only = 1 THEN 'read_only'
                  WHEN free_percent < 15 THEN 'disk'
                  WHEN connections_limit > 0 AND connections_used * 100.0 / connections_limit >= 90 THEN 'connections'
                  ELSE 'ok'
                END AS risk_reason,
                CASE
                  WHEN read_only = 1 THEN 1
                  WHEN free_percent < 15 THEN 2
                  WHEN connections_limit > 0 AND connections_used * 100.0 / connections_limit >= 90 THEN 3
                  ELSE 4
                END AS risk_rank
         FROM database_health
         WHERE environment = 'production'
       ) AS ranked
       WHERE risk_reason <> 'ok'
       ORDER BY risk_rank ASC, database_name ASC;
