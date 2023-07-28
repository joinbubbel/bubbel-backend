// @generated automatically by Diesel CLI.

diesel::table! {
    club_profiles (id) {
        id -> Int4,
        owner -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        display_name -> Nullable<Varchar>,
        pfp -> Nullable<Varchar>,
        banner -> Nullable<Varchar>,
    }
}

diesel::table! {
    user_profiles (user_id) {
        user_id -> Int4,
        name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        display_name -> Nullable<Varchar>,
        pfp -> Nullable<Varchar>,
        banner -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password_hash -> Varchar,
        email -> Varchar,
        is_verified -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    club_profiles,
    user_profiles,
    users,
);
