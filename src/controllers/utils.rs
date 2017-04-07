macro_rules! import_controller_generic_requeriments {
	($($includes:ident),*) => {
		use iron::status;
		use iron::prelude::*;
		use router::Router;

		use dal::*;

		$(
			use $includes;
		)*

		use std::io::Read;
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