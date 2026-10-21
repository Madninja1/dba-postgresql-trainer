WITH gaps AS (
         SELECT backup_id, database_name, finished_seq,
                LAG(finished_seq) OVER (PARTITION BY database_name ORDER BY finished_seq ASC, backup_id ASC) AS previous_finished_seq
         FROM backups
         WHERE status = 'success'
       )
       SELECT backup_id, database_name, finished_seq, previous_finished_seq,
              finished_seq - previous_finished_seq AS gap_seq
       FROM gaps
       WHERE finished_seq - previous_finished_seq >= 10
       ORDER BY gap_seq DESC, database_name ASC, backup_id ASC;
