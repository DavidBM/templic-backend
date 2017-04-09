use chrono::DateTime;
use chrono::UTC;
use dal::*;
use diesel;
use diesel::prelude::*;
use slog::Logger;

use dal::db_schema::users;

#[derive(Debug, Queryable, Serialize)]
pub struct User {
	pub id: i32,
	name: String,
	email: String,
	password: String,
	created_at: DateTime<UTC>,
}

impl User {
	pub fn get_by_id(user_id: i32, connection: &DbPooledConnection, logger: &Logger) -> Option<User> {
		let statement = users::table.filter(users::id.eq(user_id));

		info!(logger, "Executing Query"; "query" => debug_sql!(statement), "user_id" => user_id);

		let user = statement.load::<User>(&**connection);

		match user {
			Ok(mut user) => user.pop(),
			Err(_) => None,
		}
	}

	pub fn create(user: &NewUser, connection: &DbPooledConnection, logger: &Logger) -> Option<User> {
		let statement = diesel::insert(user)
		.into(users::table);

		info!(logger, "Executing Query"; "query" => debug_sql!(statement), "user" => format!("{:?}", user));

		let new_user = statement.get_result::<User>(&**connection);

		match new_user {
			Ok(new_user) => Some(new_user),
			Err(_) => None,
		}
	}

	pub fn delete(user_id: i32, connection: &DbPooledConnection, logger: &Logger) -> Option<u32> {
		let statement = diesel::delete(users::table.filter(users::id.eq(user_id)));

		info!(logger, "Executing Query"; "query" => debug_sql!(statement), "user_id" => user_id);

		let result = statement.execute(&**connection);
		
		match result {
			Ok(rows_deleted) => Some(rows_deleted as u32),
			Err(_) => None,
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