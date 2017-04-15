use dal::models::user::User;
use dal::db_schema::*;

#[derive(Clone, Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(User, foreign_key="user_id")]
pub struct Post {
	id: i32,
	user_id: i32,
	title: String,
	content: String
}