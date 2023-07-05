use diesel::pg::PgConnection;
use diesel::prelude::*;

mod create_user;
mod data;
mod schema;

#[cfg(test)]
mod tests;

pub use create_user::{create_user, CreateUser, CreateUserError};
pub use data::{DataState, DatabaseError, DatabaseErrorKind, User};
