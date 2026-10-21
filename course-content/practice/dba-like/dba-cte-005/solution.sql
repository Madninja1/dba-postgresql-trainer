WITH latest AS (
         SELECT database_id, MAX(finished_seq) AS finished_seq
         FROM backups
         GROUP BY database_id
       )
       SELECT d.name AS database_name, b.finished_seq, b.status
       FROM latest AS l
       JOIN backups AS b ON b.database_id = l.database_id AND b.finished_seq = l.finished_seq
       JOIN databases AS d ON d.id = b.database_id
       ORDER BY d.name ASC, b.status ASC;
