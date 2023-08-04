use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod api;
mod data;
mod schema;
mod state;

#[cfg(test)]
mod tests;

pub use api::*;
pub use data::*;
pub use state::*;
