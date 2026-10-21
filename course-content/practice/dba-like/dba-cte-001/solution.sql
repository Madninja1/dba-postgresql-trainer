SELECT d.server_name, d.name, d.size_mb
FROM databases AS d
WHERE d.size_mb > (
    SELECT AVG(peer.size_mb)
    FROM databases AS peer
    WHERE peer.server_name = d.server_name
)
ORDER BY d.server_name ASC, d.size_mb DESC, d.name ASC;
