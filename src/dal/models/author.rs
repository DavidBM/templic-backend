use dal::db_schema::authors;
use dal::models::user::User;

#[derive(Clone, Debug, Queryable, Serialize, AsChangeset, Identifiable, Associations)]
#[belongs_to(User, foreign_key="user_id")]
pub struct Author {
	#[serde(skip_serializing)]
	pub id: i32,
	pub author_id: i32,
	pub user_id: i32,
}
