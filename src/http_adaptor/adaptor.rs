use iron::prelude::*;
use mount::Mount;
use slog::Logger;

use http_adaptor::declare_endpoints;

pub struct HttpAdaptor {
	logger: Logger
}

impl HttpAdaptor {
	pub fn new(logger: Logger) -> HttpAdaptor {
		HttpAdaptor {logger: logger}
	}

	pub fn declare_endpoints(&mut self) -> Mount{
		let mut routes = Mount::new();

		declare_endpoints(&mut routes);

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
