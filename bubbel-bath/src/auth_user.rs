use super::*;
use argon2::Argon2;
use password_hash::{PasswordHash, PasswordVerifier};
use rand::{prelude::*, rngs::OsRng};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthUser {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthUserOut {
    pub token: UserToken,
    pub username: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AuthUserError {
    DatabaseError(DatabaseError),
    InvalidCredentials,
    InvalidPasswordCryto,
    UserNotFound,
}

const USER_TOKEN_LENGTH: usize = 32;

fn generate_token() -> String {
    let mut token = String::with_capacity(USER_TOKEN_LENGTH);
    let mut rng = OsRng::default();
    for _ in 0..USER_TOKEN_LENGTH {
        token.push(rng.gen_range(b'!'..b'z') as char);
    }

    token
}

pub fn auth_user(
    db: &mut DataState,
    auth: &mut AuthState,
    req: AuthUser,
) -> Result<AuthUserOut, AuthUserError> {
    use crate::schema::users::dsl;

    let (user_id, username, password_hash, email): (i32, String, String, String) = dsl::users
        .select((dsl::id, dsl::username, dsl::password_hash, dsl::email))
        .filter(dsl::username.eq(req.username))
        .first(&mut db.db)
        .map_err(|e| AuthUserError::DatabaseError(e.into()))?;
    let user_id = UserId(user_id);

    let algs: &[&dyn PasswordVerifier] = &[&Argon2::default()];
    let password =
        PasswordHash::new(&password_hash).map_err(|_| AuthUserError::InvalidPasswordCryto)?;

    password
        .verify_password(algs, &password_hash)
        .map_err(|_| AuthUserError::InvalidCredentials)?;

    let token = UserToken(generate_token());
    auth.tokens.insert(token.clone(), user_id);

    Ok(AuthUserOut {
        token,
        username,
        email,
    })
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeauthUser {
    pub token: UserToken,
}

pub fn deauth_user(auth: &mut AuthState, req: DeauthUser) {
    auth.tokens.remove_by_left(&req.token);
}
