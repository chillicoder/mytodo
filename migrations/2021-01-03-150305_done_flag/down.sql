-- This file should undo anything in `up.sql`
-- disable foreign key constraint check
PRAGMA foreign_keys=off;

  -- Here you can drop column
  CREATE TABLE IF NOT EXISTS tmp ( 
    id INTEGER NOT NULL,
    title TEXT NOT NULL,
    PRIMARY KEY (id)
    );

-- copy data from the table to the new_table
INSERT INTO tmp(id, title)
SELECT id, title
FROM task;

-- drop the table
DROP TABLE task;

-- rename the new_table to the table
ALTER TABLE tmp RENAME TO task; 

-- enable foreign key constraint check
PRAGMA foreign_keys=on;
