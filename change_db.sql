ALTER TABLE authors
  RENAME COLUMN created_at TO inserted_at;

ALTER TABLE notes
  RENAME COLUMN created_at TO inserted_at;

ALTER TABLE articles
  RENAME COLUMN created_at TO inserted_at;

ALTER TABLE pictures
  RENAME COLUMN created_at TO inserted_at;


