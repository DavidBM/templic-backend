import_controller_generic_requeriments!(serde_json);

use dal::user_model::User;
use dal::user_model::NewUser;
use controllers::utils::*;
use chrono::UTC;


pub fn get_user(req: &mut Request, connection: DbPooledConnection) -> IronResult<Response>{

	let ref user_id = req.extensions.get::<Router>().unwrap().find("id").unwrap_or("/");

	let user_id = user_id.parse::<i32>();

	let user_id = match user_id {
		Ok(user_id) => user_id,
		Err(_) => return response_bad_request("Id must be an number")
	};

	let user_data = User::get_user_by_id(user_id, &connection);

	let user_data = match user_data {
		Some(user_data) => user_data,
		None => return response_not_found("User not found")
	};

	let response_data = serde_json::to_string(&user_data).unwrap();

	response_ok(response_data)
}

pub fn create_user(req: &mut Request, connection: DbPooledConnection) -> IronResult<Response> {
	let mut payload = String::new();
	req.body.read_to_string(&mut payload).unwrap();

	let user = serde_json::from_str::<NewUser>(&payload);

	let mut user = match user {
		Ok(user) => user,
		Err(_) => return response_bad_request("Bad json")
	};

	user.created_at = Some(UTC::now().naive_utc().date());

	let user_model = User::create_user(&user, &connection);

	let user_model = match user_model {
		Some(user_model) => user_model,
		None => return response_internal_server_error("Error saving the user into db")
	};

	Ok(Response::with((
		status::Ok, 
		user_model.id.to_string()
	)))
}

pub fn delete_user(_: &mut Request, _: DbPooledConnection) -> IronResult<Response> {
	unimplemented!();
}

pub fn update_user(_: &mut Request, _: DbPooledConnection) -> IronResult<Response> {
	unimplemented!();
}

pub fn get_all_users(_: &mut Request, _: DbPooledConnection) -> IronResult<Response> {
	unimplemented!();
}
