use diesel::pg::PgConnection;
use diesel::prelude::*;

mod auth;
mod data;
mod schema;

#[cfg(test)]
mod tests;

pub use auth::AuthState;
pub use data::{DataState, User};
