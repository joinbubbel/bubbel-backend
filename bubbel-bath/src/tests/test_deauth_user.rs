use super::*;

#[test]
pub fn test_basic_deauth_user() {
    let mut db = new_data_state();
    let mut auth = AuthState::default();

    create_user(
        &mut db,
        CreateUser {
            email: "a@gmail.com".to_owned(),
            username: "usr21p1".to_owned(),
            password: "password123".to_owned(),
        },
    )
    .unwrap();

    //  TODO
    assert!(auth_user(
        &mut db,
        &mut auth,
        AuthUser {
            username: "usr21p1".to_owned(),
            password: "password".to_owned(),
        },
    )
    .is_err());

    let auth_user_res = auth_user(
        &mut db,
        &mut auth,
        AuthUser {
            username: "usr21p1".to_owned(),
            password: "password123".to_owned(),
        },
    );
    assert!(auth_user_res.is_ok());
    let auth_user_res = auth_user_res.unwrap();
    assert_eq!(auth_user_res.username, "usr21p1");
    assert_eq!(auth_user_res.email, "a@gmail.com");

    let token = auth_user_res.token;
    deauth_user(&mut auth, DeauthUser { token });
}
