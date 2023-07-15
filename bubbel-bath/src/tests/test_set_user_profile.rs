use super::*;

#[test]
#[serial_test::serial]
pub fn test_set_user_profile() {
    use crate::schema::user_profiles::dsl;

    let mut db = new_data_state();
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
    .unwrap();
    acc_limbo.push_user(acc1);

    let acc2 = create_user(
        &mut db,
        CreateUser {
            email: "pg@gmail.com".to_owned(),
            username: "davnotdev2".to_owned(),
            password: "passwordnot123".to_owned(),
        },
    )
    .unwrap();
    acc_limbo.push_user(acc2);

    waive_user_verification(&mut db, &mut acc_limbo);

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
                name: Some("Jeff".to_owned()),
                display_name: None,
                pfp: None,
                banner: None,
            },
        ),
        Err(SetUserProfileError::NoAuth)
    );

    set_user_profile(
        &mut db,
        &auth,
        SetUserProfile {
            token: acc1.clone(),
            name: Some("David Zhong".to_owned()),
            display_name: None,
            pfp: None,
            banner: None,
        },
    )
    .unwrap();

    set_user_profile(
        &mut db,
        &mut auth,
        SetUserProfile {
            token: acc1.clone(),
            name: None,
            display_name: None,
            pfp: Some("https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_owned()),
            banner: None,
        },
    )
    .unwrap();

    let mut user_profiles = dsl::user_profiles.load::<UserProfile>(&mut db.db).unwrap();
    user_profiles.sort_by(|a, b| a.user_id.cmp(&b.user_id));

    assert_eq!(
        user_profiles,
        vec![
            UserProfile {
                user_id: 1,
                name: Some("David Zhong".to_owned()),
                display_name: None,
                pfp: Some("https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_owned()),
                banner: None
            },
            UserProfile {
                user_id: 2,
                name: None,
                display_name: None,
                pfp: None,
                banner: None
            }
        ]
    );
}
