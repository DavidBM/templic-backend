macro_rules! import_controller_generic_requeriments {
	($($includes:ident),*) => {
		use iron::prelude::*;
		#[allow(unused_imports)]
		use router::Router;

		use slog::Logger;

		use dal::*;

		$(
			use $includes;
		)*
		#[allow(unused_imports)]
		use std::io::Read;
		use controllers::utils::*;
	}
}

use iron::status;
use iron::prelude::*;

pub fn response_not_found<S: Into<String>>(text: S) -> IronResult<Response> {
	return Ok(Response::with((
		status::NotFound, 
		text.into()
	)));
}

pub fn response_bad_request<S: Into<String>>(text: S) -> IronResult<Response> {
	return Ok(Response::with((
		status::BadRequest, 
		text.into()
	)));
}

pub fn response_internal_server_error<S: Into<String>>(text: S) -> IronResult<Response> {
	return Ok(Response::with((
		status::InternalServerError, 
		text.into()
	)));
}

pub fn response_ok<S: Into<String>>(text: S) -> IronResult<Response> {
	return Ok(Response::with((
		status::Ok, 
		text.into()
	)));
}