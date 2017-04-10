use std::env;
use slog::*;
use slog_term;
use std;
use slog_json::Json;
use std::sync::Mutex;

use iron::{typemap, BeforeMiddleware};
use iron::prelude::*;


pub struct LoggerMiddleware {
	pub logger: Logger
}

impl LoggerMiddleware {
	pub fn new () -> LoggerMiddleware{	

		let log_output_type = env::var("LOG_OUTPUT").expect("LOG_OUTPUT must be set");

		match log_output_type.as_ref() {
		    "json" => {
		    	let json = Mutex::new(Json::default(std::io::stdout())).map(Fuse);

				LoggerMiddleware {logger: Logger::root(
					json,
					o!("app" => "templic-backend")
				)}
		    },
		    _ => {
				let plain = slog_term::PlainSyncDecorator::new(std::io::stdout());
				let drain = slog_term::FullFormat::new(plain).build().fuse();

				LoggerMiddleware {logger: Logger::root(
					drain,
					o!("app" => "templic-backend")
				)}
		    }
		}
	}

	pub fn get_logger(&self) -> Logger {
		self.logger.clone()
	}
}

pub struct Value(Logger);

impl typemap::Key for LoggerMiddleware { type Value = Value; }

impl BeforeMiddleware for LoggerMiddleware {
	fn before(&self, req: &mut Request) -> IronResult<()> {
		let logger = self.logger.new(o!("route" => format!("{}", req.url)));
		req.extensions.insert::<LoggerMiddleware>(Value(logger));
		Ok(())
	}
}

pub trait LoggerReqExt {
	fn get_logger(&self) -> Logger;
}

impl <'a, 'b>LoggerReqExt for Request <'a, 'b> {
	fn get_logger(&self) -> Logger {
		let &Value(ref logger) = self.extensions.get::<LoggerMiddleware>().unwrap();

		logger.clone()
	}
}
