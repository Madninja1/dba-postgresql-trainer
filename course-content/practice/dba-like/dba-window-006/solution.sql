WITH samples AS (
         SELECT sample_id, database_name, sampled_seq, duration_ms,
                LAG(duration_ms) OVER (PARTITION BY database_name ORDER BY sampled_seq ASC, sample_id ASC) AS previous_duration_ms
         FROM query_stats
       )
       SELECT sample_id, database_name, sampled_seq, duration_ms, previous_duration_ms,
              duration_ms - previous_duration_ms AS delta_ms
       FROM samples
       ORDER BY database_name ASC, sampled_seq ASC, sample_id ASC;
