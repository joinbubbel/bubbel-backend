use super::*;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub email: String,
}

const USERNAME_MIN_LENGTH: usize = 5;
const USERNAME_MAX_LENGTH: usize = 15;

fn validate_username(username: &str) -> bool {
    let chars = username.chars().collect::<Vec<_>>();
    username.is_ascii()
        && chars.len() >= USERNAME_MIN_LENGTH
        && chars.len() <= USERNAME_MAX_LENGTH
        && !chars.into_iter().all(|c| c.is_alphanumeric() && c == ' ')
}

const PASSWORD_MIN_LENGTH: usize = 5;
const PASSWORD_MAX_LENGTH: usize = 15;

fn validate_password(password: &str) -> bool {
    let chars = password.chars().collect::<Vec<_>>();
    password.is_ascii()
        && chars.len() >= PASSWORD_MIN_LENGTH
        && chars.len() <= PASSWORD_MAX_LENGTH
        && !chars.into_iter().all(|c| c.is_alphanumeric() && c == ' ')
}

pub struct CreateUser {
    username: String,
    password: String,
}

pub enum CreateUserError {
    DatabaseError,
    InvalidUsername,
    InvalidPassword,
}

pub fn create_user(db: &DataState, req: CreateUser) -> Result<(), CreateUserError> {
    use crate::schema::users::dsl::*;

    validate_username(&req.username)
        .then_some(())
        .ok_or(CreateUserError::InvalidUsername)?;
    validate_password(&req.password)
        .then_some(())
        .ok_or(CreateUserError::InvalidPassword)?;

    todo!()
}
