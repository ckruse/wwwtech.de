ALTER TABLE articles ADD COLUMN posse_visibility TEXT NOT NULL DEFAULT 'public';
ALTER TABLE articles ADD COLUMN content_warning CHARACTER VARYING(255);

ALTER TABLE deafies ADD COLUMN posse_visibility TEXT NOT NULL DEFAULT 'public';
ALTER TABLE deafies ADD COLUMN content_warning CHARACTER VARYING(255);

ALTER TABLE pictures ADD COLUMN posse_visibility TEXT NOT NULL DEFAULT 'public';
ALTER TABLE pictures ADD COLUMN content_warning CHARACTER VARYING(255);

ALTER TABLE notes ADD COLUMN posse_visibility TEXT NOT NULL DEFAULT 'public';
ALTER TABLE notes ADD COLUMN content_warning CHARACTER VARYING(255);
