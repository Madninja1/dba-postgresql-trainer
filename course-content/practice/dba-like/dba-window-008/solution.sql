WITH edges AS (
         SELECT database_name,
                FIRST_VALUE(duration_ms) OVER (
                  PARTITION BY database_name ORDER BY sampled_seq ASC, sample_id ASC
                  ROWS BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING
                ) AS first_duration_ms,
                LAST_VALUE(duration_ms) OVER (
                  PARTITION BY database_name ORDER BY sampled_seq ASC, sample_id ASC
                  ROWS BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING
                ) AS last_duration_ms
         FROM query_stats
       )
       SELECT DISTINCT database_name, first_duration_ms, last_duration_ms,
              last_duration_ms - first_duration_ms AS delta_ms
       FROM edges
       ORDER BY ABS(delta_ms) DESC, database_name ASC;
