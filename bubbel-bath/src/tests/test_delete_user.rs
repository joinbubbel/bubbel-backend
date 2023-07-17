use super::*;

#[test]
#[serial_test::serial]
pub fn test_delete_user() {
    use crate::schema::users::dsl;

    let mut db = new_data_state();
    let mut auth = AuthState::default();
    let mut acc_limbo = AccountLimboState::default();

    let acc1 = create_user(
        &mut db,
        CreateUser {
            email: "d@gmail.com".to_owned(),
            username: "davnotdev".to_owned(),
            password: "abcdef123".to_owned(),
        },
    )
    .unwrap()
    .user_id;
    acc_limbo.push_user(acc1);

    let acc2 = create_user(
        &mut db,
        CreateUser {
            email: "g@gmail.com".to_owned(),
            username: "davnotdev2".to_owned(),
            password: "abcdef123".to_owned(),
        },
    )
    .unwrap()
    .user_id;
    acc_limbo.push_user(acc2);

    acc_limbo.waive_user_verification(&mut db);

    let acc = auth_user(
        &mut db,
        &mut auth,
        AuthUser {
            username: "davnotdev".to_owned(),
            password: "abcdef123".to_owned(),
        },
    )
    .unwrap()
    .token;

    delete_user(&mut db, &mut auth, DeleteUser { token: acc.clone() }).unwrap();

    let usernames: Vec<String> = dsl::users
        .select(dsl::username)
        .get_results(&mut db.db)
        .unwrap();
    assert_eq!(usernames, vec!["davnotdev2"]);

    assert_eq!(
        delete_user(&mut db, &mut auth, DeleteUser { token: acc }),
        Err(DeleteUserError::NoAuth),
    );

    assert_eq!(
        auth_user(
            &mut db,
            &mut auth,
            AuthUser {
                username: "davnotdev".to_owned(),
                password: "abcdef123".to_owned()
            }
        ),
        Err(AuthUserError::UserNotFound),
    );
}
