use iron::prelude::*;
use router::Router;
use mount::Mount;
use slog::Logger;

use dal::*;
use controllers::test_controller;
use controllers::user_controller;

macro_rules! declare_endpoint {
	($router:expr, $pool:expr, $logger:expr, $route:expr, $method:ident, $name:expr, $lambda:expr) => {
		{
			let logger = $logger.new(o!("route" => $route));
			let query_pool = $pool.clone();
			$router.$method($route, move |req: &mut Request| -> IronResult<Response> {
				info!(logger, "Endpoint Call");
				$lambda(req, query_pool.get().unwrap(), &logger)
			}, $name);
		}
	}
}

macro_rules! declare_multiple_endpoints {
	($routes_name:expr, $pool:expr, $logger:expr, $main_router:expr, $mount_route:expr, $( $name:expr => $method:ident, $route:expr, $handler:expr ),*) => {
		{
			let mut sub_router = Router::new();
			let logger = $logger.new(o!("controller" => $routes_name, "main_route" => $mount_route));
			$(
				declare_endpoint!(sub_router, $pool, logger, $route, $method, $name, $handler);
			)*
			$main_router.mount($mount_route, sub_router);
		}
	}
}

pub fn declare_endpoints(pool: DbConnectionPool, logger: Logger) -> Mount {
	let mut routes = Mount::new();

	declare_multiple_endpoints!(
		"test", pool, logger, routes, "/",
		"ping" => get, "/ping", test_controller::ping
	);

	declare_multiple_endpoints!(
		"user", pool, logger, routes, "/user/",
		"get_user" => get, "/:id", user_controller::get_user,
		"delete_user" => delete, "/:id", user_controller::delete_user,
		"update_user" => put, "/:id", user_controller::update_user,
		"create_user" => post, "/", user_controller::create_user,
		"get_all_user" => get, "/", user_controller::get_all_users
	);

	routes
}
