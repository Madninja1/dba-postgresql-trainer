SELECT server_name, SUM(size_gb) AS total_size_gb
       FROM databases
       GROUP BY server_name
       HAVING SUM(size_gb) >= 100
       ORDER BY total_size_gb DESC, server_name ASC;
