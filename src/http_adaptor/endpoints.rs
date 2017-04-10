use router::Router;
use mount::Mount;
use iron::prelude::Chain;

use controllers::test_controller;
use controllers::user_controller;

macro_rules! declare_multiple_endpoints {
	($main_router:expr, $mount_route:expr, $( $name:expr => $method:ident ; $route:expr ; [$($middleware_before:expr),*] => $handler:expr => [$($middleware_after:expr),*]),*) => {
		{
			let mut sub_router = Router::new();
			$(
				{

					let chain = Chain::new($handler);

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

pub fn declare_endpoints(routes: &mut Mount) {
	declare_multiple_endpoints!(
		routes, "/",
		"ping" => get; "/ping"; [] => test_controller::ping => []
	);

	declare_multiple_endpoints!(
		routes, "/user/",
		"get_user" => get; "/:id"; [] => user_controller::get_user => [],
		"delete_user" => delete; "/:id"; [] => user_controller::delete_user => [],
		"update_user" => put; "/:id"; [] => user_controller::update_user => [],
		"create_user" => post; "/"; [] => user_controller::create_user => []
	);
}
