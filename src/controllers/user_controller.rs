import_controller_generic_requeriments!();

use dal::UserModels::{User, UpdateUser};

pub fn get_user(req: &mut Request) -> IronResult<Response>{
	let connection = req.get_db_conn();
	let logger = req.get_logger();

	let user_id = get_route_parameter_as!(i32, req, "id", response_not_found("User not found"));

	let user_data = some_or_return!(
		User::get_by_id(user_id, &connection, &logger), 
		response_not_found("User not found")
	);

	response_ok(&user_data)
}

pub fn get_me(req: &mut Request) -> IronResult<Response> {
	response_ok(&req.get_user_data())
}

pub fn delete_user(req: &mut Request) -> IronResult<Response> {
	let connection = req.get_db_conn();
	let logger = req.get_logger();

	let user_id = get_route_parameter_as!(i32, req, "id", response_not_found("User not found"));

	let quatity_deleted = ok_or_return!(
		User::delete(user_id, &connection, &logger),
		response_internal_server_error("Error deleting the user")
	);

	info!(logger, "Deleted users"; "quatity_deleted" => quatity_deleted);

	response_ok(&json!({"quantity": quatity_deleted}))
}

pub fn update_user(req: &mut Request) -> IronResult<Response> {
	let connection = req.get_db_conn();
	let logger = req.get_logger();

	let user_id = get_route_parameter_as!(i32, req, "id", response_not_found("User not found"));
	let user = get_body_as!(UpdateUser, req, response_bad_request);

	let user = ok_or_return!(
		User::update(&user, user_id, &connection, &logger),
		response_internal_server_error("Error deleting the user")
	);

	response_ok(&user)
}
