import_controller_generic_requeriments!();

pub fn ping(_: &mut Request, _: DbPooledConnection, _: &Logger) -> IronResult<Response>{
	response_ok_text("pong")
}