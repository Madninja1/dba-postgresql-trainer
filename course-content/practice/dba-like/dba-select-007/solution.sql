SELECT role_name, valid_until_seq
       FROM db_roles
       WHERE can_login = 1 AND valid_until_seq < 100
       ORDER BY valid_until_seq ASC, role_name ASC;
