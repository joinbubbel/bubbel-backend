use super::*;
use argon2::Argon2;
use password_hash::{PasswordHasher, SaltString};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct AuthUser {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct AuthUserOut {
    pub token: UserToken,
    pub username: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum AuthUserError {
    InvalidCredentials,
    /// Got an error from a cryptography function.
    /// This error should never occur.
    InvalidPasswordCryto,
    UserNotFound,
    UserNotVerified {
        user_id: UserId,
    },
    Internal {
        ierror: String,
    },
}

pub async fn auth_user(
    db: &mut DataStateInstance,
    auth: &mut AuthState,
    req: AuthUser,
) -> Result<AuthUserOut, AuthUserError> {
    use crate::schema::users::dsl;

    let (user_id, username, password_hash, email, is_verified): (
        i32,
        String,
        String,
        String,
        bool,
    ) = dsl::users
        .select((
            dsl::id,
            dsl::username,
            dsl::password_hash,
            dsl::email,
            dsl::is_verified,
        ))
        .filter(dsl::username.eq(req.username))
        .first(&mut db.db)
        .map_err(|e| match DatabaseError::from(e) {
            DatabaseError::NotFound => AuthUserError::UserNotFound,
            e => AuthUserError::Internal {
                ierror: e.to_string(),
            },
        })?;
    let user_id = UserId(user_id);
    is_verified
        .then_some(())
        .ok_or(AuthUserError::UserNotVerified { user_id })?;

    let salt = SaltString::from_b64(&db.user_salt).unwrap();
    let argon2 = Argon2::default();
    let password = argon2
        .hash_password(&req.password.into_bytes(), &salt)
        .map_err(|_| AuthUserError::InvalidPasswordCryto)?
        .to_string();

    (password == password_hash)
        .then_some(())
        .ok_or(AuthUserError::InvalidCredentials)?;

    let token = auth.unchecked_auth_user(&user_id);

    Ok(AuthUserOut {
        token,
        username,
        email,
    })
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct DeauthUser {
    pub token: UserToken,
}

pub async fn deauth_user(auth: &mut AuthState, req: DeauthUser) {
    auth.deauth_user(&req.token);
}
