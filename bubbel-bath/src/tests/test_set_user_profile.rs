use super::*;

#[test]
#[serial_test::serial]
pub fn test_set_user_profile() {
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

    let acc1 = auth_user(
        &mut db,
        &mut auth,
        AuthUser {
            username: "davnotdev".to_owned(),
            password: "passwordnot123".to_owned(),
        },
    )
    .unwrap()
    .token;

    let _ = auth_user(
        &mut db,
        &mut auth,
        AuthUser {
            username: "davnotdev2".to_owned(),
            password: "passwordnot123".to_owned(),
        },
    )
    .unwrap()
    .token;

    assert_eq!(
        set_user_profile(
            &mut db,
            &auth,
            SetUserProfile {
                token: UserToken("donuts".to_owned()),
                profile: UserProfilePartial {
                    name: Some("Jeff".to_owned()),
                    ..Default::default()
                }
            },
        ),
        Err(SetUserProfileError::NoAuth)
    );

    set_user_profile(
        &mut db,
        &auth,
        SetUserProfile {
            token: acc1.clone(),
            profile: UserProfilePartial {
                name: Some("David Zhong".to_owned()),
                ..Default::default()
            },
        },
    )
    .unwrap();

    set_user_profile(
        &mut db,
        &auth,
        SetUserProfile {
            token: acc1.clone(),
            profile: UserProfilePartial {
                pfp: Some("https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_owned()),
                ..Default::default()
            },
        },
    )
    .unwrap();

    assert_eq!(
        get_user_profile(
            &mut db,
            &auth,
            GetUserProfile {
                user_id: UserId(99),
                token: None
            }
        ),
        Err(GetUserProfileError::UserNotFound)
    );

    assert_eq!(
        get_user_profile(
            &mut db,
            &auth,
            GetUserProfile {
                user_id: UserId(1),
                token: None
            }
        ),
        Ok(GetUserProfileOut {
            name: Some("David Zhong".to_owned()),
            description: None,
            display_name: None,
            pfp: Some("https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_owned()),
            banner: None
        })
    );

    assert_eq!(
        get_user_profile(
            &mut db,
            &auth,
            GetUserProfile {
                user_id: UserId(2),
                token: None
            }
        ),
        Ok(GetUserProfileOut {
            name: None,
            description: None,
            display_name: None,
            pfp: None,
            banner: None
        })
    );
}
