use iron::status;
use iron::prelude::*;

use dal::*;

pub fn ping(_: &mut Request, _: DbPooledConnection) -> IronResult<Response>{
	Ok(Response::with((status::Ok, "pong")))
}
