use super::*;
use argon2::Argon2;
use password_hash::{PasswordHash, PasswordHasher, SaltString};
use rand::rngs::OsRng;

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

fn validate_email(email: &str) -> bool {
    //  https://emailregex.com/
    //  TODO OPT: Cache regex expression?.
    let re = regex::Regex::new(r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#).unwrap();
    re.find(email).is_some()
}

#[derive(Debug, Clone)]
pub struct CreateUser {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreateUserError {
    /// Got an error from database.
    DatabaseError(DatabaseError),
    /// Email is not valid by backend standards.
    InvalidEmail,
    /// Username is not valid by backend standards.
    InvalidUsername,
    /// Password is not valid by backend standards.
    InvalidPassword,
    /// Password failed to be encrypted.
    InvalidPasswordCryto,
}

pub fn create_user(db: &mut DataState, req: CreateUser) -> Result<(), CreateUserError> {
    use crate::schema::users::dsl;

    validate_username(&req.username)
        .then_some(())
        .ok_or(CreateUserError::InvalidUsername)?;
    validate_password(&req.password)
        .then_some(())
        .ok_or(CreateUserError::InvalidPassword)?;
    validate_email(&req.email)
        .then_some(())
        .ok_or(CreateUserError::InvalidEmail)?;

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password = argon2
        .hash_password(&req.password.into_bytes(), &salt)
        .map_err(|_| CreateUserError::InvalidPasswordCryto)?
        .to_string();
    let password = PasswordHash::new(&password)
        .map_err(|_| CreateUserError::InvalidPasswordCryto)?
        .to_string();

    let new_user = User {
        username: req.username,
        password_hash: password,
        email: req.email,
    };

    diesel::insert_into(dsl::users)
        .values(&new_user)
        .execute(&mut db.db)
        .map_err(|e| CreateUserError::DatabaseError(e.into()))?;

    Ok(())
}
