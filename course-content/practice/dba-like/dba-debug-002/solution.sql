SELECT d.name AS database_name, MAX(b.finished_seq) AS finished_seq
FROM databases AS d
LEFT JOIN backups AS b
  ON b.database_id = d.id
 AND b.status = 'success'
GROUP BY d.id, d.name
ORDER BY d.name ASC;
