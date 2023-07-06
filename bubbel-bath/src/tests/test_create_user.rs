use super::*;

#[test]
pub fn test_create_multiple_users() {
    let mut db = new_data_state();
    assert_eq!(
        create_user(
            &mut db,
            CreateUser {
                email: "ggmail.com".to_owned(),
                username: "fklakfdsb".to_owned(),
                password: "gfjkdlsfjkds".to_owned(),
            },
        ),
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
        ),
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
        ),
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
        ),
        Ok(())
    );
    assert_eq!(
        create_user(
            &mut db,
            CreateUser {
                email: "g@gmail.com".to_owned(),
                username: "fskdjfklakfdsb".to_owned(),
                password: "gfjkdlsfjkds".to_owned(),
            },
        ),
        Ok(())
    );
    assert_eq!(
        create_user(
            &mut db,
            CreateUser {
                email: "g@gmail.com".to_owned(),
                username: "fskdjfklakfdsb".to_owned(),
                password: "gfjkdlsfjkds".to_owned(),
            },
        ),
        Err(CreateUserError::DatabaseError(DatabaseError::Database(
            DatabaseErrorKind::UniqueViolation
        )))
    );
}