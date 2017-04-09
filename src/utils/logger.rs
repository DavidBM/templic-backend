use slog::*;
//use slog_term;
use std;
use slog_json::Json;
use std::sync::Mutex;

pub fn get_main_logger() -> Logger {
	//let plain = slog_term::PlainSyncDecorator::new(std::io::stdout());
	let json = Mutex::new(Json::default(std::io::stdout())).map(Fuse);
	//let drain = slog_term::FullFormat::new(plain).build().fuse();

	Logger::root(
		json,
		o!("app" => "templic-backend")
	)
}
