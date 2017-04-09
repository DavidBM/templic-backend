use chrono::DateTime;
use chrono::UTC;
use dal::*;
use diesel;
use diesel::prelude::*;
use slog::Logger;

use dal::db_schema::users;

#[derive(Debug, Queryable, Serialize, AsChangeset, Identifiable)]
pub struct User {
	pub id: i32,
	name: String,
	email: String,
	password: String,
	created_at: DateTime<UTC>,
}

impl User {
	pub fn get_by_id(user_id: i32, connection: &DieselConnection, logger: &Logger) -> Option<User> {
		let statement = users::table.filter(users::id.eq(user_id));

		info!(logger, "Executing Query"; "query" => debug_sql!(statement), "user_id" => user_id);

		let user = statement.load::<User>(&**connection);

		match user {
			Ok(mut user) => user.pop(),
			Err(_) => None,
		}
	}

	pub fn create(user: &NewUser, connection: &DieselConnection, logger: &Logger) -> Result<User, diesel::result::Error> {
		let statement = diesel::insert(user)
		.into(users::table);

		info!(logger, "Executing Query"; "query" => debug_sql!(statement), "user" => format!("{:?}", user));

		let new_user = statement.get_result::<User>(&**connection);

		match new_user {
			Ok(new_user) => Ok(new_user),
			Err(error) => Err(error),
		}
	}

	pub fn update(user: &UpdateUser, user_id:i32, connection: &DieselConnection, logger: &Logger) -> Result<User, diesel::result::Error> {
		let mut user_in_db = some_or_return!(
			User::get_by_id(user_id, connection, logger),
			Err(diesel::result::Error::NotFound)
		);

		if let Some(name) = user.name.clone() {
			user_in_db.name = name;
		}

		if let Some(email) = user.email.clone() {
			user_in_db.email = email;
		}

		if let Some(password) = user.password.clone() {
			user_in_db.password = password;
		}

		let result = user_in_db.save_changes::<User>(&**connection);

		match result {
			Ok(_) => Ok(user_in_db),
			Err(error) => Err(error),
		}
	}

	pub fn delete(user_id: i32, connection: &DieselConnection, logger: &Logger) -> Result<u32, diesel::result::Error> {
		let statement = diesel::delete(users::table.filter(users::id.eq(user_id)));

		info!(logger, "Executing Query"; "query" => debug_sql!(statement), "user_id" => user_id);

		let result = statement.execute(&**connection);
		
		match result {
			Ok(rows_deleted) => Ok(rows_deleted as u32),
			Err(error) => Err(error),
		}
	}
}


#[derive(Debug, Insertable, Deserialize)]
#[table_name="users"]
pub struct NewUser {
	name: String,
	email: String,
	password: String,
	pub created_at: Option<DateTime<UTC>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser {
	name: Option<String>,
	email: Option<String>,
	password: Option<String>
}
