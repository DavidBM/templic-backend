# Templic backend

Written in Rust.

Rust components:
- slog: log system
- r2d2: database connection pool
- diesel: ORM
- iron: http framework
- serde: json en/decoder

## Structure

Routes are declared in `src/http_adaptor/endpoints`. Some macros are used for automate the code there.

The Diesel (PostgreSQL) ORM connections are pooled with r2d2, meaning that all the Iron threats can do queries at the same time. Every request is a thread (following the Iron framework model).

The output is a JSON version of the database model using Serde json.

This code use macros in several places, use `cargo expand` (`cargo install cargo-expand`) for see the final code.

## Dependencies

- rust and cargo (Install using [https://www.rustup.rs/](https://www.rustup.rs/))
- diesel-cli - Install: `cargo install diesel_cli`
- PostgreSQL

Compiled using Rust `rustc 1.16.0 (30cf806ef 2017-03-10)` stable version. 

## Before run

- Update the database connection data in the file `.env`
- `cd rust-webserver-demo`
- `diesel migration run`

## Running

- `cargo run`

## Creating new database migrations

- `diesel migration generate <migration name>`
- fill the files `up.sql` and `down.sql`
- check with `diesel migration run`
- check again with `diesel migration redo`

If there isn't errors, you finish! :)

## Adding new endpoints 

- Add the function to the controller or create a new controller in `src/controllers`.
	+ If a new controller is created, don't forget to add it in `src/controllers/mod.rs`.
- Add the function to `src/http_adaptor/endpoints.rs`.
	+ Maybe you need to add `use controllers::<controller file name>;` at the beginning of the file.
- Fill the controller function
- Test it

## Recommended packages for working in Rust & sublime text editor

- Sublime packages
	+ rust enhanced
	+ anaconda_rust
	+ sublimeLinter-contrib-rustc
- `cargo install cargo-expand`
- `cargo install cargo-watch`