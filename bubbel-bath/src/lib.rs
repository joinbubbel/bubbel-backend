use diesel::pg::PgConnection;
use diesel::prelude::*;

mod auth;
mod auth_user;
mod create_user;
mod data;
mod schema;

#[cfg(test)]
mod tests;

pub use auth::{AuthState, UserId, UserToken};
pub use auth_user::{auth_user, deauth_user, AuthUser, AuthUserError, AuthUserOut, DeauthUser};
pub use create_user::{create_user, CreateUser, CreateUserError};
pub use data::{DataState, DatabaseError, DatabaseErrorKind, User};
