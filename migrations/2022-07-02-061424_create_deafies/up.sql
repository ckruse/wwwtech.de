CREATE TABLE deafies (
  id serial PRIMARY KEY,
  author_id integer NOT NULL REFERENCES authors (id),
  title character varying NOT NULL,
  slug character varying NOT NULL,
  guid character varying NOT NULL,
  excerpt text,
  body text NOT NULL,
  published boolean NOT NULL DEFAULT FALSE,
  inserted_at timestamp without time zone NOT NULL,
  updated_at timestamp without time zone NOT NULL
);

CREATE INDEX deafies_author_id_idx ON deafies (author_id);

