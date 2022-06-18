DROP TABLE IF EXISTS todos;
DROP TABLE IF EXISTS template;

CREATE TABLE todos (
  id serial PRIMARY KEY,
  note TEXT NOT NULL
);

CREATE TABLE template (
  id serial PRIMARY KEY,
  html TEXT NOT NULL
);


INSERT INTO todos (note) VALUES ('Buy milk');
INSERT INTO todos (note) VALUES ('Buy eggs');
INSERT INTO todos (note) VALUES ('Buy bread');



