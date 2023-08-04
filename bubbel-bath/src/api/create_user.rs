use super::*;
use argon2::Argon2;
use password_hash::{PasswordHasher, SaltString};

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

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct CreateUser {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct CreateUserOut {
    pub user_id: UserId,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum CreateUserError {
    /// Email is not valid by backend standards.
    InvalidEmail,
    /// Username is not valid by backend standards.
    InvalidUsername,
    /// Password is not valid by backend standards.
    InvalidPassword,
    /// Got an error from a cryptography function.
    /// This error should never occur.
    InvalidPasswordCryto,
    /// Email or Username already taken.
    EmailOrUsernametaken,
    Internal {
        ierror: String,
    },
}

pub fn create_user(db: &mut DataStateInstance, req: CreateUser) -> Result<CreateUserOut, CreateUserError> {
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

    let salt = SaltString::from_b64(&db.user_salt).unwrap();
    let argon2 = Argon2::default();
    let password = argon2
        .hash_password(&req.password.into_bytes(), &salt)
        .map_err(|_| CreateUserError::InvalidPasswordCryto)?
        .to_string();

    let new_user = User {
        username: req.username.clone(),
        password_hash: password,
        email: req.email,
        is_verified: false,
    };

    diesel::insert_into(dsl::users)
        .values(&new_user)
        .execute(&mut db.db)
        .map_err(|e| match DatabaseError::from(e) {
            DatabaseError::UniqueViolation => CreateUserError::EmailOrUsernametaken,
            e => CreateUserError::Internal {
                ierror: e.to_string(),
            },
        })?;

    let ids = dsl::users
        .select(dsl::id)
        .filter(dsl::username.eq(req.username))
        .load::<i32>(&mut db.db)
        .map_err(|e| CreateUserError::Internal {
            ierror: e.to_string(),
        })?;
    assert_eq!(ids.len(), 1);
    let id = UserId(*ids.first().unwrap());

    UserProfile::insert_new(db, &id).map_err(|e| CreateUserError::Internal {
        ierror: e.to_string(),
    })?;

    Ok(CreateUserOut { user_id: id })
}
