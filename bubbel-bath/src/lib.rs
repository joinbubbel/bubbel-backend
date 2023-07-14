use diesel::pg::PgConnection;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod account_verification;
mod auth;
mod auth_user;
mod create_user;
mod data;
mod schema;
mod set_user_profile;

#[cfg(test)]
mod tests;

pub(crate) use account_verification::waive_user_verification;

pub use account_verification::{verify_user, AccountLimboState, VerifyAccount, VerifyAccountError};
pub use auth::{generate_token_alphanumeric, AuthState, UserId, UserToken};
pub use auth_user::{auth_user, deauth_user, AuthUser, AuthUserError, AuthUserOut, DeauthUser};
pub use create_user::{create_user, CreateUser, CreateUserError};
pub use data::{DataState, DatabaseError, User, UserProfile};
pub use set_user_profile::{set_user_profile, SetUserProfile, SetUserProfileError};
