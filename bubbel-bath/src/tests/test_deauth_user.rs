use super::*;

#[tokio::test]
#[serial_test::serial]
pub async fn test_basic_deauth_user() {
    let dbs = new_data_state();
    let mut db = dbs.spawn();
    let mut auth = AuthState::default();
    let mut acc_limbo = AccountLimboState::default();

    let acc = create_user(
        &mut db,
        CreateUser {
            email: "a@gmail.com".to_owned(),
            username: "usr21p1".to_owned(),
            password: "password123".to_owned(),
        },
    )
    .await
    .unwrap()
    .user_id;
    acc_limbo.push_user(acc);

    acc_limbo.waive_user_verification(&mut db).await;

    assert_eq!(
        auth_user(
            &mut db,
            &mut auth,
            AuthUser {
                username: "usr21p1".to_owned(),
                password: "password".to_owned(),
            },
        )
        .await,
        Err(AuthUserError::InvalidCredentials)
    );

    assert_eq!(
        auth_user(
            &mut db,
            &mut auth,
            AuthUser {
                username: "usr21p".to_owned(),
                password: "password123".to_owned(),
            },
        )
        .await,
        Err(AuthUserError::UserNotFound)
    );

    let auth_user_res = auth_user(
        &mut db,
        &mut auth,
        AuthUser {
            username: "usr21p1".to_owned(),
            password: "password123".to_owned(),
        },
    )
    .await;
    assert!(auth_user_res.is_ok());
    let auth_user_res = auth_user_res.unwrap();
    assert_eq!(auth_user_res.username, "usr21p1");
    assert_eq!(auth_user_res.email, "a@gmail.com");

    let token = auth_user_res.token;
    deauth_user(&mut auth, DeauthUser { token }).await;
}
