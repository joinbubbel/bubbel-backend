use super::*;

#[test]
#[serial_test::serial]
pub fn test_join_clubs() {
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

    let club_id = create_club(
        &mut db,
        &auth,
        CreateClub {
            token: acc1_token.clone(),
            name: "Acc1 Club".to_owned(),
        },
    )
    .unwrap()
    .club_id;

    assert_eq!(
        unjoin_club(
            &mut db,
            &auth,
            UnjoinClub {
                token: acc1_token.clone(),
                club_id,
            },
        ),
        Err(UnjoinClubError::CannotUnjoinAsOwner)
    );

    join_club(
        &mut db,
        &auth,
        JoinClub {
            token: acc2_token.clone(),
            club_id,
        },
    )
    .unwrap();

    assert_eq!(
        join_club(
            &mut db,
            &auth,
            JoinClub {
                token: acc2_token.clone(),
                club_id,
            },
        ),
        Err(JoinClubError::AlreadyJoined)
    );

    unjoin_club(
        &mut db,
        &auth,
        UnjoinClub {
            token: acc2_token.clone(),
            club_id,
        },
    )
    .unwrap();

    assert_eq!(
        get_club_members(&mut db, GetClubMembers { club_id })
            .unwrap()
            .users,
        vec![acc1]
    );
}