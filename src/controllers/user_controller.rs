import_controller_generic_requeriments!(serde_json);

use dal::user_model::User;
use dal::user_model::NewUser;
use chrono::UTC;

pub fn get_user(req: &mut Request, connection: DbPooledConnection, logger: &Logger) -> IronResult<Response>{

	let user_id = get_route_parameter_as!(i32, req, "id", response_not_found("User not found"));

	let user_data = some_or_return!(
		User::get_by_id(user_id, &connection, logger), 
		response_not_found("User not found")
	);

	response_ok(&user_data)
}

pub fn create_user(req: &mut Request, connection: DbPooledConnection, logger: &Logger) -> IronResult<Response> {
	let mut user = get_body_as!(req, NewUser, response_bad_request);

	user.created_at = Some(UTC::now());

	let user_model = User::create(&user, &connection, logger);

	let user_model = some_or_return!(
		user_model, 
		response_internal_server_error("Error saving the user into db")
	);

	response_ok(&json!({"id": user_model.id}))
}

pub fn delete_user(req: &mut Request, connection: DbPooledConnection, logger: &Logger) -> IronResult<Response> {
	let user_id = get_route_parameter_as!(i32, req, "id", response_not_found("User not found"));

	let quatity_deleted = some_or_return!(
		User::delete(user_id, &connection, logger),
		response_internal_server_error("Error deleting the user")
	);

	info!(logger, "Deleted users"; "quatity_deleted" => quatity_deleted);

	response_ok(&json!({"quantity": quatity_deleted}))
}

pub fn update_user(_: &mut Request, _: DbPooledConnection, _: &Logger) -> IronResult<Response> {
	unimplemented!();
}

pub fn get_all_users(_: &mut Request, _: DbPooledConnection, _: &Logger) -> IronResult<Response> {
	unimplemented!();
}
