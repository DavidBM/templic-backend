use chrono::NaiveDate;
use dal::*;
use diesel;
use diesel::prelude::*;

use dal::db_schema::users;

#[derive(Debug, Queryable, Serialize)]
pub struct User {
	pub id: i32,
	name: String,
	email: String,
	password: String,
	created_at: NaiveDate,
}

impl User {
	pub fn get_user_by_id(user_id: i32, connection: &DbPooledConnection) -> Option<User> {
		let result = users::table
		.filter(users::id.eq(user_id))
		.load::<User>(&**connection);

		if let Ok(mut result) = result {
			return result.pop()
		}

		None
	}

	pub fn create_user(user: &NewUser, connection: &DbPooledConnection) -> Option<User> {
		let new_user = diesel::insert(user)
		.into(users::table)
		.get_result::<User>(&**connection);

		if let Ok(new_user) = new_user {
			return Some(new_user)
		}

		None
	}
}

#[derive(Debug, Insertable, Deserialize)]
#[table_name="users"]
pub struct NewUser {
	name: String,
	email: String,
	password: String,
	pub created_at: Option<NaiveDate>,
}