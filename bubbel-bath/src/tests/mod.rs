use super::{schema::users::dsl::*, *};
use diesel::{pg::PgConnection, prelude::*};

pub fn new_test_database() -> Option<PgConnection> {
    let pg = "postgresql://postgres:abc@localhost:5432";
    PgConnection::establish(pg).ok()
}

#[test]
pub fn test() {
    let Some(mut conn) = new_test_database() else {
        eprintln!("Early disconnect, test database not running.");
        return;
    };
    let _ = users.select(User::as_select()).load(&mut conn).unwrap();
}
