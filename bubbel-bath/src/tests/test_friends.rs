use super::*;

#[test]
#[serial_test::serial]
pub fn test_friends() {
    let dbs = new_data_state();
    let mut db = dbs.spawn();
    let mut auth = AuthState::default();
    let mut acc_limbo = AccountLimboState::default();

    let acc1 = create_user(
        &mut db,
        CreateUser {
            email: "g@gmail.com".to_owned(),
            username: "davnotdev".to_owned(),
            password: "passwordnot123".to_owned(),
        },
    )
    .unwrap()
    .user_id;
    acc_limbo.push_user(acc1);

    let acc2 = create_user(
        &mut db,
        CreateUser {
            email: "pg@gmail.com".to_owned(),
            username: "davnotdev2".to_owned(),
            password: "passwordnot123".to_owned(),
        },
    )
    .unwrap()
    .user_id;
    acc_limbo.push_user(acc2);

    let acc3 = create_user(
        &mut db,
        CreateUser {
            email: "ppg@gmail.com".to_owned(),
            username: "davnotdev3".to_owned(),
            password: "passwordnot123".to_owned(),
        },
    )
    .unwrap()
    .user_id;
    acc_limbo.push_user(acc3);

    acc_limbo.waive_user_verification(&mut db);

    let acc1_token = auth_user(
        &mut db,
        &mut auth,
        AuthUser {
            username: "davnotdev".to_owned(),
            password: "passwordnot123".to_owned(),
        },
    )
    .unwrap()
    .token;

    let acc2_token = auth_user(
        &mut db,
        &mut auth,
        AuthUser {
            username: "davnotdev2".to_owned(),
            password: "passwordnot123".to_owned(),
        },
    )
    .unwrap()
    .token;

    let acc3_token = auth_user(
        &mut db,
        &mut auth,
        AuthUser {
            username: "davnotdev3".to_owned(),
            password: "passwordnot123".to_owned(),
        },
    )
    .unwrap()
    .token;

    add_friend_connection(
        &mut db,
        &auth,
        AddFriendConnection {
            token: acc1_token.clone(),
            receiver_id: acc3,
        },
    )
    .unwrap();

    assert_eq!(
        add_friend_connection(
            &mut db,
            &auth,
            AddFriendConnection {
                token: acc1_token.clone(),
                receiver_id: acc3,
            },
        ),
        Err(AddFriendConnectionError::AlreadyConnected)
    );

    assert_eq!(
        add_friend_connection(
            &mut db,
            &auth,
            AddFriendConnection {
                token: acc1_token.clone(),
                receiver_id: acc1,
            },
        ),
        Err(AddFriendConnectionError::CannotAddSelf)
    );

    add_friend_connection(
        &mut db,
        &auth,
        AddFriendConnection {
            token: acc3_token.clone(),
            receiver_id: acc1,
        },
    )
    .unwrap();

    add_friend_connection(
        &mut db,
        &auth,
        AddFriendConnection {
            token: acc2_token.clone(),
            receiver_id: acc1,
        },
    )
    .unwrap();

    let acc1_conns = get_friend_connections(
        &mut db,
        &auth,
        GetFriendConnection {
            token: acc1_token.clone(),
        },
    )
    .unwrap()
    .friend_connections;
    assert_eq!(acc1_conns.get(&acc3), Some(&FriendStatus::Full));
    assert_eq!(acc1_conns.get(&acc2), Some(&FriendStatus::RecievedPending));

    let acc2_conns = get_friend_connections(
        &mut db,
        &auth,
        GetFriendConnection {
            token: acc2_token.clone(),
        },
    )
    .unwrap()
    .friend_connections;
    assert_eq!(acc2_conns.get(&acc1), Some(&FriendStatus::SentPending));

    let acc3_conns = get_friend_connections(
        &mut db,
        &auth,
        GetFriendConnection {
            token: acc3_token.clone(),
        },
    )
    .unwrap()
    .friend_connections;
    assert_eq!(acc3_conns.get(&acc1), Some(&FriendStatus::Full));

    remove_friend(
        &mut db,
        &auth,
        RemoveFriend {
            token: acc1_token.clone(),
            removal_id: acc3,
        },
    )
    .unwrap();

    let acc1_conns = get_friend_connections(
        &mut db,
        &auth,
        GetFriendConnection {
            token: acc1_token.clone(),
        },
    )
    .unwrap()
    .friend_connections;
    assert_eq!(acc1_conns.get(&acc2), Some(&FriendStatus::RecievedPending));

    let acc2_conns =
        get_friend_connections(&mut db, &auth, GetFriendConnection { token: acc2_token })
            .unwrap()
            .friend_connections;
    assert_eq!(acc2_conns.get(&acc1), Some(&FriendStatus::SentPending));

    let acc3_conns =
        get_friend_connections(&mut db, &auth, GetFriendConnection { token: acc3_token })
            .unwrap()
            .friend_connections;
    assert!(acc3_conns.is_empty());
}
