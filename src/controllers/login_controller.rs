import_controller_generic_requeriments!();

use std::env;

use chrono::UTC;
use argon2rs::argon2i_simple;
use base64::encode;
use jwt::{encode as encode_jwt, Algorithm, Header};

use dal::UserModels::{User, NewUser};
use http_adaptor::apis::Login;
use middlewares::get_salt::GetSaltReqExt;

pub fn login(req: &mut Request) -> IronResult<Response>{
	let login_data = get_body_as!(Login, req, response_bad_request);
	let connection = req.get_db_conn();
	let logger = req.get_logger();

	let user_data = some_or_return!(
		User::get_user_by_email_or_name(&login_data, &connection, &logger), 
		response_not_found("User or password incorrect")
	);
	
	let salt = create_user_salt(&req.get_salt(), &user_data);

	let hash = argon2i_simple(login_data.password.as_ref(), salt.as_ref());
	let encoded_password = encode(&hash);

	if user_data.password != encoded_password {
		return response_not_found("User or password incorrect");
	}

	let token = create_token(&user_data);

	response_ok_text(token)
}

pub fn register(req: &mut Request) -> IronResult<Response> {
	let mut user = get_body_as!(NewUser, req, response_bad_request);
	let connection = req.get_db_conn();
	let logger = req.get_logger();

	user.created_at = Some(UTC::now());

	let salt = req.get_salt();

	let hash = argon2i_simple(user.password.as_ref(), salt);
	let encoded = encode(&hash);

	user.password = encoded;

	let user_model = ok_or_return!(
		User::create(&user, &connection, &logger), 
		response_internal_server_error("Error saving the user into db")
	);

	response_ok(&json!({"id": user_model.id}))
}

fn create_token(user_data: &User) -> String {
	let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

	let mut header = Header::default();
	header.alg = Algorithm::HS512;
	encode_jwt(header, user_data, secret.as_ref()).unwrap()
}

fn create_user_salt(static_salt: &String, user: &User) -> String {
	let time = user.created_at.timestamp_subsec_millis().to_be();

	(
		time.count_ones() 
		+ time.leading_zeros() 
		+ time 
		+ time.count_zeros().to_le() 
		+ time.rotate_left(5)
	).to_string() 
	+ static_salt
}