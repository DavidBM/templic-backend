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
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod dal;
mod http_adaptor;
mod controllers;

use iron::prelude::Iron;
use dotenv::dotenv;

use dal::Db;
use http_adaptor::endpoints::declare_endpoints;

fn main() {
	dotenv().ok();

	let db = Db::new();
	let db_connection_pool = db.get_pool();
	let router = declare_endpoints(db_connection_pool);

	println!("Server running in localhost:3000");

	Iron::new(router).http("localhost:3000").unwrap();
}