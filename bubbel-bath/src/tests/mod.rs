use super::{schema::users::dsl::*, *};
use diesel::{pg::PgConnection, prelude::*};

pub fn new_test_database() -> PgConnection {
    let pg = "postgresql://postgres:abc@localhost:5432";
    PgConnection::establish(pg).unwrap()
}

#[test]
pub fn test() {
    let mut conn = new_test_database();
    let _ = users.select(User::as_select()).load(&mut conn).unwrap();
}
