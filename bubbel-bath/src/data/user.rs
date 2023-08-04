use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct UserId(pub i32);

#[derive(Queryable, Selectable, Insertable, Debug, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub username: String,
    pub password_hash: String,
    pub email: String,
    pub is_verified: bool,
}

impl User {
    /// Try to get a user using `id`.
    pub fn get(db: &mut DataStateInstance, id: UserId) -> Result<Option<Self>, DatabaseError> {
        use crate::schema::users::dsl;

        dsl::users
            .select((
                dsl::username,
                dsl::password_hash,
                dsl::email,
                dsl::is_verified,
            ))
            .filter(dsl::id.eq(id.0))
            .load::<User>(&mut db.db)
            .map(|v| v.first().cloned())
            .map_err(DatabaseError::from)
    }

    /// Try to get a user using `username`.
    pub fn get_user_id_with_username(
        db: &mut DataStateInstance,
        username: &str,
    ) -> Result<Option<UserId>, DatabaseError> {
        use crate::schema::users::dsl;

        dsl::users
            .select(dsl::id)
            .filter(dsl::username.eq(username))
            .load::<i32>(&mut db.db)
            .map(|u| u.first().map(|&u| UserId(u)))
            .map_err(DatabaseError::from)
    }

    /// Remove a user using `id`.
    /// Nothing happens if the user doesn't exist.
    pub fn remove(db: &mut DataStateInstance, id: UserId) -> Result<(), DatabaseError> {
        use crate::schema::users::dsl;

        diesel::delete(dsl::users)
            .filter(dsl::id.eq(id.0))
            .execute(&mut db.db)
            .map(|_| ())
            .map_err(DatabaseError::from)
    }
}
