use super::*;

#[tokio::test]
#[serial_test::serial]
pub async fn test_friends() {
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
    .await
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
    .await
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
    .await
    .unwrap()
    .user_id;
    acc_limbo.push_user(acc3);

    acc_limbo.waive_user_verification(&mut db).await;

    let acc1_token = auth_user(
        &mut db,
        &mut auth,
        AuthUser {
            username: "davnotdev".to_owned(),
            password: "passwordnot123".to_owned(),
        },
    )
    .await
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
    .await
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
    .await
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
    .await
    .unwrap();

    assert_eq!(
        add_friend_connection(
            &mut db,
            &auth,
            AddFriendConnection {
                token: acc1_token.clone(),
                receiver_id: acc3,
            },
        )
        .await,
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
        )
        .await,
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
    .await
    .unwrap();

    add_friend_connection(
        &mut db,
        &auth,
        AddFriendConnection {
            token: acc2_token.clone(),
            receiver_id: acc1,
        },
    )
    .await
    .unwrap();

    let acc1_conns = get_friend_connections(
        &mut db,
        &auth,
        GetFriendConnections {
            token: acc1_token.clone(),
        },
    )
    .await
    .unwrap()
    .friend_connections;
    assert_eq!(acc1_conns.get(&acc3), Some(&FriendStatus::Full));
    assert_eq!(acc1_conns.get(&acc2), Some(&FriendStatus::RecievedPending));

    let acc2_conns = get_friend_connections(
        &mut db,
        &auth,
        GetFriendConnections {
            token: acc2_token.clone(),
        },
    )
    .await
    .unwrap()
    .friend_connections;
    assert_eq!(acc2_conns.get(&acc1), Some(&FriendStatus::SentPending));

    let acc3_conns = get_friend_connections(
        &mut db,
        &auth,
        GetFriendConnections {
            token: acc3_token.clone(),
        },
    )
    .await
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
    .await
    .unwrap();

    let acc1_conns = get_friend_connections(
        &mut db,
        &auth,
        GetFriendConnections {
            token: acc1_token.clone(),
        },
    )
    .await
    .unwrap()
    .friend_connections;
    assert_eq!(acc1_conns.get(&acc2), Some(&FriendStatus::RecievedPending));

    let acc2_conns =
        get_friend_connections(&mut db, &auth, GetFriendConnections { token: acc2_token })
            .await
            .unwrap()
            .friend_connections;
    assert_eq!(acc2_conns.get(&acc1), Some(&FriendStatus::SentPending));

    let acc3_conns =
        get_friend_connections(&mut db, &auth, GetFriendConnections { token: acc3_token })
            .await
            .unwrap()
            .friend_connections;
    assert!(acc3_conns.is_empty());
}
