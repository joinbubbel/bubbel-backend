use diesel::pg::PgConnection;
use diesel::prelude::*;

mod account_verification;
mod auth;
mod auth_user;
mod create_user;
mod data;
mod schema;

#[cfg(test)]
mod tests;

pub(crate) use account_verification::waive_user_verification;

pub use account_verification::{verify_user, AccountLimboState, VerifyAccount, VerifyAccountError};
pub use auth::{AuthState, UserId, UserToken};
pub use auth_user::{
    auth_user, deauth_user, generate_token_alphanumeric, AuthUser, AuthUserError, AuthUserOut,
    DeauthUser,
};
pub use create_user::{create_user, CreateUser, CreateUserError};
pub use data::{DataState, DatabaseError, User};
pub use serde::{Deserialize, Serialize};
