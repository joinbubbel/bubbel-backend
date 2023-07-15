use super::*;

#[derive(Queryable, Selectable, Insertable, AsChangeset, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = crate::schema::user_profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserProfile {
    pub user_id: i32,
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub pfp: Option<String>,
    pub banner: Option<String>,
}

impl UserProfile {
    pub fn insert_new(db: &mut DataState, user_id: &UserId) -> Result<(), DatabaseError> {
        use crate::schema::user_profiles::dsl;

        diesel::insert_into(dsl::user_profiles)
            .values(&UserProfile {
                user_id: user_id.0,
                name: None,
                display_name: None,
                pfp: None,
                banner: None,
            })
            .execute(&mut db.db)
            .map(|_| ())
            .map_err(DatabaseError::from)
    }

    pub fn update_partial(&self, db: &mut DataState) -> Result<(), DatabaseError> {
        use crate::schema::user_profiles::dsl;

        diesel::update(dsl::user_profiles)
            .set(self)
            .filter(dsl::user_id.eq(self.user_id))
            .execute(&mut db.db)
            .map(|_| ())
            .map_err(DatabaseError::from)
    }
}
