use super::*;

#[tokio::test]
#[serial_test::serial]
pub async fn test_create_multiple_users() {
    let dbs = new_data_state();
    let mut db = dbs.spawn();
    assert_eq!(
        create_user(
            &mut db,
            CreateUser {
                email: "ggmail.com".to_owned(),
                username: "fklakfdsb".to_owned(),
                password: "gfjkdlsfjkds".to_owned(),
            },
        )
        .await,
        Err(CreateUserError::InvalidEmail)
    );
    assert_eq!(
        create_user(
            &mut db,
            CreateUser {
                email: "g@gmail.com".to_owned(),
                username: "a".to_owned(),
                password: "gfjkdlsfjkds".to_owned(),
            },
        )
        .await,
        Err(CreateUserError::InvalidUsername)
    );
    assert_eq!(
        create_user(
            &mut db,
            CreateUser {
                email: "g@gmail.com".to_owned(),
                username: "akdfsljkdfjks".to_owned(),
                password: "a b".to_owned(),
            },
        )
        .await,
        Err(CreateUserError::InvalidPassword)
    );
    assert_eq!(
        create_user(
            &mut db,
            CreateUser {
                email: "d@gmail.com".to_owned(),
                username: "abcjkdsf".to_owned(),
                password: "gfdjksafdljksa".to_owned(),
            },
        )
        .await,
        Ok(CreateUserOut { user_id: UserId(1) })
    );
    assert_eq!(
        create_user(
            &mut db,
            CreateUser {
                email: "g@gmail.com".to_owned(),
                username: "fskdjfklakfdsb".to_owned(),
                password: "gfjkdlsfjkds".to_owned(),
            },
        )
        .await,
        Ok(CreateUserOut { user_id: UserId(2) })
    );
    assert_eq!(
        create_user(
            &mut db,
            CreateUser {
                email: "other@gmail.com".to_owned(),
                username: "fskdjfklakfdsb".to_owned(),
                password: "gfjkdlsfjkds".to_owned(),
            },
        )
        .await,
        Err(CreateUserError::EmailOrUsernameTaken)
    );

    assert_eq!(
        create_user(
            &mut db,
            CreateUser {
                email: "g@gmail.com".to_owned(),
                username: "abcdef".to_owned(),
                password: "gfjkdlsfjkds".to_owned(),
            },
        )
        .await,
        Err(CreateUserError::EmailOrUsernameTaken)
    );
}
