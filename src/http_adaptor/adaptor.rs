use iron::prelude::*;
use mount::Mount;

use http_adaptor::declare_endpoints;

use dal::DieselMiddleware;
use utils::LoggerMiddleware;


pub struct HttpAdaptor;

impl HttpAdaptor {
	pub fn new() -> HttpAdaptor {
		HttpAdaptor
	}

	pub fn declare_endpoints(&mut self) -> Mount{
		let mut routes = Mount::new();

		declare_endpoints(&mut routes);

		routes
	}

	pub fn create_chain(&self, routes: Mount) -> Chain {
		let mut chain = Chain::new(routes);

		self.add_default_middlewares(&mut chain);

		chain
	}

	fn add_default_middlewares(&self, chain: &mut Chain) {
		let db_pool_middleware = DieselMiddleware::new();
		let logger_middleware = LoggerMiddleware::new();

		chain.link_before(logger_middleware);
		chain.link_before(db_pool_middleware);
	}

	pub fn start_http(&self, chain: Chain, host: &str, port: &str) {
		let address = format!("{}:{}", host, port);
		
		{
			let logger = LoggerMiddleware::new();
			info!(logger.get_logger(), "Server Running"; o!("address" => address.clone()));
		}

		Iron::new(chain).http(address).unwrap();
	}
}
