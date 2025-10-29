// @generated automatically by Diesel CLI.

diesel::table! {
    rust_post (id) {
        id -> BigInt,
        #[max_length = 255]
        title -> Varchar,
        content -> Longtext,
        published -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    rust_user (id) {
        id -> BigInt,
        #[max_length = 20]
        name -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(rust_post, rust_user,);
