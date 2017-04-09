use iron::prelude::*;
use router::Router;
use mount::Mount;
use slog::Logger;

use controllers::test_controller;
use controllers::user_controller;

macro_rules! declare_multiple_endpoints {
	($routes_name:expr, $main_router:expr, $mount_route:expr, $( $name:expr => $method:ident, $route:expr, $handler:expr ),*) => {
		{
			let mut sub_router = Router::new();
			$(
				sub_router.$method($route, $handler, $name);
			)*
			$main_router.mount($mount_route, sub_router);
		}
	}
}

pub struct HttpAdaptor {
	logger: Logger
}

impl HttpAdaptor {
	pub fn new(logger: Logger) -> HttpAdaptor {
		HttpAdaptor {logger: logger}
	}

	pub fn declare_endpoints(&mut self) -> Mount{
		let mut routes = Mount::new();

		declare_multiple_endpoints!(
			"test", routes, "/",
			"ping" => get, "/ping", test_controller::ping
		);

		declare_multiple_endpoints!(
			"user", routes, "/user/",
			"get_user" => get, "/:id", user_controller::get_user,
			"delete_user" => delete, "/:id", user_controller::delete_user,
			"update_user" => put, "/:id", user_controller::update_user,
			"create_user" => post, "/", user_controller::create_user
		);

		routes
	}

	pub fn create_chain(&self, routes: Mount) -> Chain {
		Chain::new(routes)
	}

	pub fn start_http(&self, chain: Chain, host: &str, port: &str) {
		let address = format!("{}:{}", host, port);

		info!(self.logger, "Server running"; o!("address" => address.clone()));

		Iron::new(chain).http(address).unwrap();
	}
}
