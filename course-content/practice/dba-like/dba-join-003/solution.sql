SELECT d.name AS database_name, b.id AS backup_id, b.finished_seq
FROM databases AS d
LEFT JOIN backups AS b
  ON b.database_id = d.id
 AND b.status = 'success'
LEFT JOIN backups AS newer
  ON newer.database_id = b.database_id
 AND newer.status = 'success'
 AND (newer.finished_seq > b.finished_seq
      OR (newer.finished_seq = b.finished_seq AND newer.id > b.id))
WHERE newer.id IS NULL
ORDER BY d.name ASC;
