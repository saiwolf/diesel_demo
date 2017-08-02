CREATE TABLE posts (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f',
  user_id INTEGER NOT NULL
)

CREATE TABLE `users` (
	`id`	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	`email`	varchar NOT NULL,
	`password`	VARCHAR NOT NULL
);