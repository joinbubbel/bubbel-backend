use super::*;

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub username: String,
    pub password_hash: String,
    pub email: String,
}
