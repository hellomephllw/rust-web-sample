use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Identifiable, AsChangeset, Clone)]
#[diesel(table_name = crate::schema::rust_post)]
pub struct Post {
    id: i64,
    title: String,
    content: String,
    published: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}
