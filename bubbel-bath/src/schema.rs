// @generated automatically by Diesel CLI.

diesel::table! {
    club_members (id) {
        id -> Int4,
        user_id -> Int4,
        club_id -> Int4,
    }
}

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
    friend_connections (id) {
        id -> Int4,
        send_user_id -> Int4,
        recv_user_id -> Int4,
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
    club_members,
    club_profiles,
    friend_connections,
    user_profiles,
    users,
);
