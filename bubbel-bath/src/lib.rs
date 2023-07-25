use diesel::pg::PgConnection;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod api;
mod data;
mod schema;
mod state;

#[cfg(test)]
mod tests;

pub use data::{DataState, DatabaseError, User, UserProfile};

pub use api::*;
pub use state::*;
