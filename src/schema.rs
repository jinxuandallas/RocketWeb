// @generated automatically by Diesel CLI.

diesel::table! {
    person (ID) {
        ID -> Integer,
        #[max_length = 45]
        name -> Varchar,
        age -> Integer,
    }
}

diesel::table! {
    user (ID) {
        ID -> Integer,
        #[max_length = 45]
        username -> Varchar,
        #[max_length = 200]
        password -> Varchar,
        #[max_length = 250]
        token -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    person,
    user,
);
