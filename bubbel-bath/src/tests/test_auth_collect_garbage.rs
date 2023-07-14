use super::*;

#[test]
#[serial_test::serial]
pub fn test_auth_collect_garbage() {
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

    auth.check_user_with_token(&acc1);

    let acc2 = auth_user(
        &mut db,
        &mut auth,
        AuthUser {
            username: "davnotdev2".to_owned(),
            password: "passwordnot123".to_owned(),
        },
    )
    .unwrap()
    .token;

    auth.collect_garbage_with_expire(std::time::Duration::from_secs(0));

    assert!(auth.check_user_with_token(&acc1).is_some());
    assert!(auth.check_user_with_token(&acc2).is_none());
}
