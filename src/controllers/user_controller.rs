import_controller_generic_requeriments!(serde_json);

use dal::user_model::User;
use dal::user_model::NewUser;
use dal::user_model::UpdateUser;
use chrono::UTC;

pub fn get_user(req: &mut Request, logger: &Logger) -> IronResult<Response>{
	let connection = req.db_conn();
	
	let user_id = get_route_parameter_as!(i32, req, "id", response_not_found("User not found"));

	let user_data = some_or_return!(
		User::get_by_id(user_id, &connection, logger), 
		response_not_found("User not found")
	);

	response_ok(&user_data)
}

pub fn create_user(req: &mut Request, logger: &Logger) -> IronResult<Response> {
	let connection = req.db_conn();

	let mut user = get_body_as!(NewUser, req, response_bad_request);

	user.created_at = Some(UTC::now());

	let user_model = ok_or_return!(
		User::create(&user, &connection, logger), 
		response_internal_server_error("Error saving the user into db")
	);

	response_ok(&json!({"id": user_model.id}))
}

pub fn delete_user(req: &mut Request, logger: &Logger) -> IronResult<Response> {
	let connection = req.db_conn();

	let user_id = get_route_parameter_as!(i32, req, "id", response_not_found("User not found"));

	let quatity_deleted = ok_or_return!(
		User::delete(user_id, &connection, logger),
		response_internal_server_error("Error deleting the user")
	);

	info!(logger, "Deleted users"; "quatity_deleted" => quatity_deleted);

	response_ok(&json!({"quantity": quatity_deleted}))
}

pub fn update_user(req: &mut Request, logger: &Logger) -> IronResult<Response> {
	let connection = req.db_conn();

	let user_id = get_route_parameter_as!(i32, req, "id", response_not_found("User not found"));
	let user = get_body_as!(UpdateUser, req, response_bad_request);

	let user = ok_or_return!(
		User::update(&user, user_id, &connection, logger),
		response_internal_server_error("Error deleting the user")
	);

	response_ok(&user)
}

