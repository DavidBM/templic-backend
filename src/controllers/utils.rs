macro_rules! import_controller_generic_requeriments {
	($($includes:ident),*) => {
		use iron::prelude::*;
		#[allow(unused_imports)]
		use router::Router;

		use slog::Logger;

		#[allow(unused_imports)]
		use dal::*;
		#[allow(unused_imports)]
		use dal::DieselReqExt;

		$(
			use $includes;
		)*
		#[allow(unused_imports)]
		use std::error::Error;
		#[allow(unused_imports)]
		use std::io::Read;
		use controllers::utils::*;
		#[allow(unused_imports)]
		use utils::macros;
	}
}

macro_rules! create_http_response {
	($name:ident, $status:expr, "to_json_error") => {
		#[allow(dead_code)]
		pub fn $name<S: Into<String>>(text: S) -> IronResult<Response> {
			return Ok(Response::with((
				$status, 
				json!({"error": text.into()}).to_string()
			)));
		}
	};
	($name:ident, $status:expr, "to_json") => {
		#[allow(dead_code)]
		pub fn $name<S: Serialize>(response: &S) -> IronResult<Response> {
			let json_text = serde_json::to_string(response).unwrap();
			return Ok(Response::with((
				$status, 
				json_text
			)));
		}
	};
	($name:ident, $status:expr, "text") => {
		#[allow(dead_code)]
		pub fn $name<S: Into<String>>(text: S) -> IronResult<Response> {
			return Ok(Response::with((
				$status, 
				text.into()
			)));
		}
	};
}

macro_rules! get_body_as {
	($structure:ty, $req:expr, $error_fn:ident) => {
		{
			let body = get_body!($req, $error_fn);

			let structure = serde_json::from_str::<$structure>(&body);

			match structure {
				Ok(structure) => structure,
				Err(error) => return $error_fn(format!("{}: {}", error.description(), error))
			}
		}
	}
}

macro_rules! get_body {
	($req:expr, $error_fn:ident) => {
		{
			let mut payload = String::new();

			if let Err(_) = $req.body.read_to_string(&mut payload) {
				return $error_fn("Request body not found")
			}

			payload
		}
	}
}

macro_rules! get_route_parameter_as {
	($parse_type:ty, $req:expr, $param:expr, $return_http:expr) => {
		{
			let ref param = get_route_parameter!($req, $param, $return_http);

			match param.parse::<$parse_type>() {
				Ok(expr) => expr,
				Err(_) => return $return_http
			}
		}
	}
}

macro_rules! get_route_parameter {
	($req:expr, $param:expr, $return_http:expr) => {
		{
			let param = $req.extensions.get::<Router>().unwrap().find($param);

			some_or_return!(param, $return_http)
		}
	}
}

use iron::status;
use iron::prelude::*;
use serde::ser::Serialize;
use serde_json;
use iron::status::Status;


#[allow(dead_code)]
pub fn response_text<S: Into<String>>(text: S, status:Status) -> IronResult<Response> {
	return Ok(Response::with((
		status, 
		text.into()
	)));
}

create_http_response!(response_ok, status::Ok, "to_json");
create_http_response!(response_ok_text, status::Ok, "text");

create_http_response!(response_not_found, status::NotFound, "to_json_error");
create_http_response!(response_bad_request, status::BadRequest, "to_json_error");
create_http_response!(response_internal_server_error, status::InternalServerError, "to_json_error");
