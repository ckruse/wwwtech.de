# WWWTech.de

This is my personal website, running at <https://wwwtech.de>.

It is an [Actix](https://actix.rs) project, using [diesel](https://diesel.rs) for communicating with a
[PostgreSQL](https://www.postgresql.org) database and [Askama](https://github.com/djc/askama) for rendering HTML.

## Running

See `.env` for configuration during development (like pathes and database config). Then run `cargo run` and visit
the endpoint at localhost:8080 (no protocol to avoid autolinking).
