// @generated automatically by Diesel CLI.

diesel::table! {
    rust_user (id) {
        id -> Integer,
        #[max_length = 20]
        name -> Varchar,
    }
}
