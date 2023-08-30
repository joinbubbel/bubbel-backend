use super::*;

#[tokio::test]
#[serial_test::serial]
pub async fn test_account_collect_garbage() {
    use crate::schema::users::dsl;

    let dbs = new_data_state();
    let mut db = dbs.spawn();
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
    let acc1_code = acc_limbo.push_user(acc1);

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
    let _ = acc_limbo.push_user(acc2);

    verify_account(&mut db, &mut acc_limbo, VerifyAccount { code: acc1_code })
        .await
        .unwrap();

    acc_limbo.collect_garbage_with_expire(&mut db, std::time::Duration::from_secs(0));

    let usernames: Vec<String> = dsl::users
        .select(dsl::username)
        .get_results(&mut db.db)
        .unwrap();

    assert_eq!(usernames, vec!["davnotdev"])
}
