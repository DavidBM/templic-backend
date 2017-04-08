use slog_term;
use slog::Logger;
use slog::*;
use std;

pub fn get_main_logger() -> Logger {
	let plain = slog_term::PlainSyncDecorator::new(std::io::stdout());
	let drain = slog_term::FullFormat::new(plain).build().fuse();

	Logger::root(
		drain,
		o!("app" => "templic-backend")
	)
}
