extern crate iron;
extern crate router;
extern crate mount;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate dotenv;
extern crate chrono;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_json;

#[macro_use]
mod utils;
mod dal;
mod controllers;
mod http_adaptor;

use iron::prelude::Iron;
use dotenv::dotenv;

use dal::Db;
use http_adaptor::endpoints::declare_endpoints;
use utils::logger::get_main_logger;

fn main() {
	dotenv().ok();

	let log = get_main_logger();

	let db = Db::new();
	let db_connection_pool = db.get_pool();
	let router = declare_endpoints(db_connection_pool, log);

	println!("Server running in localhost:3000");

	Iron::new(router).http("localhost:3000").unwrap();
}