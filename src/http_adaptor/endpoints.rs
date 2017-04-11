use router::Router;
use mount::Mount;
use iron::prelude::Chain;
use slog::Logger;

use controllers::test_controller;
use controllers::user_controller;

use middlewares::LoginMiddleware;

macro_rules! declare_multiple_endpoints {
	($main_router:expr, $mount_route:expr, $( $name:expr => $method:ident ; $route:expr ; [$($middleware_before:expr),*] => $handler:expr => [$($middleware_after:expr),*]),*) => {
		{
			let mut sub_router = Router::new();
			$(
				{
					#![allow(unused_mut)]
					let mut chain = Chain::new($handler);

					$(
						chain.link_before($middleware_before);
					)*

					$(
						chain.link_before($middleware_after);
					)*

					sub_router.$method($route, chain, $name);
				}
			)*
			$main_router.mount($mount_route, sub_router);
		}
	}
}

pub fn declare_endpoints(routes: &mut Mount, logger: &Logger) {
	let loggin = LoginMiddleware::new(&logger);

	declare_multiple_endpoints!(
		routes, "/",
		"ping" => get; "/ping"; [] => test_controller::ping => [],
		"read_login_user" => get; "/read_login_user"; [loggin] => test_controller::read_login_user => []
	);

	declare_multiple_endpoints!(
		routes, "/user/",
		"get_user" => get; "/:id"; [] => user_controller::get_user => [],
		"delete_user" => delete; "/:id"; [] => user_controller::delete_user => [],
		"update_user" => put; "/:id"; [] => user_controller::update_user => [],
		"create_user" => post; "/"; [] => user_controller::create_user => []
	);
}
