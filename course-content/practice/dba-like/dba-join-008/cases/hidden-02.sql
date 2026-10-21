CREATE TABLE databases(id INTEGER, name TEXT); CREATE TABLE backups(id INTEGER, database_id INTEGER, status TEXT);
         INSERT INTO databases VALUES (1,'a'),(2,'b'),(3,'c'),(4,'d'); INSERT INTO backups VALUES (1,1,'success'),(2,3,'success');
