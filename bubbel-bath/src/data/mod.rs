use diesel::pg::PgConnection;
use diesel::prelude::*;

pub struct DataState {
    db: PgConnection,
}

impl DataState {
    pub fn new(db: PgConnection) -> Self {
        DataState { db }
    }
}
