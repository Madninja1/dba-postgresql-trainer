SELECT server_name, database_name, size_gb,
              DENSE_RANK() OVER (PARTITION BY server_name ORDER BY size_gb DESC) AS size_rank
       FROM databases
       ORDER BY server_name ASC, size_rank ASC, database_name ASC;
