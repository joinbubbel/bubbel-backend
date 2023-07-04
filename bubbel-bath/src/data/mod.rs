use super::*;

mod user;

pub use user::User;

pub struct DataState {
    db: PgConnection,
}

impl DataState {
    pub fn new(db: PgConnection) -> Self {
        DataState { db }
    }
}
