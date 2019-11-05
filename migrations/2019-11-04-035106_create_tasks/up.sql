-- -*- sql-product:postgres -*-

CREATE TABLE tasks (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL DEFAULT '',
  due_date TIMESTAMP,
  completed BOOLEAN NOT NULL DEFAULT 'f'
)
