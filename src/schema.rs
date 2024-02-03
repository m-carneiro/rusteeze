use diesel::{allow_tables_to_appear_in_same_query, joinable, table};

table! {
    login_history (id) {
        id -> Uuid,
        user_id -> Uuid,
        login_time -> Timestamptz,
        logout_time -> Timestamptz,
    }
}

table! {
    people (id) {
        id -> Int4,
        name -> Varchar,
        gender -> Bool,
        age -> Int4,
        address -> Varchar,
        phone -> Varchar,
        email -> Varchar,
    }
}

table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        password -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        is_active -> Bool,
        is_superuser -> Bool,
        login_session -> Varchar,
    }
}

joinable!(login_history -> users (user_id));

allow_tables_to_appear_in_same_query!(login_history, people, users);