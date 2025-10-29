use diesel::{AsChangeset, Identifiable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Identifiable, AsChangeset, Clone)]
#[diesel(table_name = crate::schema::rust_user)]
// #[diesel(check_for_backend(diesel::mysql::Mysql))]
struct User {
    id: i32,
    name: String,
}
