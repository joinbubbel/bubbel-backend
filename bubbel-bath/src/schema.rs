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
        dc_id -> Int4,
        description -> Nullable<Varchar>,
        display_name -> Nullable<Varchar>,
        pfp -> Nullable<Varchar>,
        banner -> Nullable<Varchar>,
    }
}

diesel::table! {
    data_channels (id) {
        id -> Int4,
        data_channel -> Varchar,
    }
}

diesel::table! {
    data_chunks (id) {
        id -> Int4,
        data_chunk -> Varchar,
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
    message_group_members (user_id) {
        user_id -> Int4,
        message_group_id -> Int4,
    }
}

diesel::table! {
    message_groups (message_group_id) {
        message_group_id -> Int4,
        name -> Nullable<Varchar>,
        dc_id -> Int4,
    }
}

diesel::table! {
    message_room_members (user_id) {
        user_id -> Int4,
        message_room_id -> Int4,
    }
}

diesel::table! {
    message_rooms (message_room_id) {
        message_room_id -> Int4,
        name -> Nullable<Varchar>,
        club_id -> Int4,
        dc_id -> Int4,
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

diesel::joinable!(message_group_members -> message_groups (message_group_id));
diesel::joinable!(message_room_members -> message_rooms (message_room_id));
diesel::joinable!(message_rooms -> club_profiles (club_id));

diesel::allow_tables_to_appear_in_same_query!(
    club_members,
    club_profiles,
    data_channels,
    data_chunks,
    friend_connections,
    message_group_members,
    message_groups,
    message_room_members,
    message_rooms,
    user_profiles,
    users,
);
