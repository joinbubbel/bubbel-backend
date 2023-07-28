use super::*;

#[derive(Queryable, Selectable, Insertable, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = crate::schema::user_profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserProfile {
    pub user_id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub display_name: Option<String>,
    pub pfp: Option<String>,
    pub banner: Option<String>,
}

impl UserProfile {
    /// Inserts an empty user profile with `user_id`.
    /// This function should only be called in [`create_user`].
    pub fn insert_new(db: &mut DataState, user_id: &UserId) -> Result<(), DatabaseError> {
        use crate::schema::user_profiles::dsl;

        diesel::insert_into(dsl::user_profiles)
            .values(&UserProfile {
                user_id: user_id.0,
                name: None,
                description: None,
                display_name: None,
                pfp: None,
                banner: None,
            })
            .execute(&mut db.db)
            .map(|_| ())
            .map_err(DatabaseError::from)
    }

    /// Try to get a user profile using `id`.
    pub fn get(
        db: &mut DataState,
        id: UserId,
    ) -> Result<Option<UserProfile>, DatabaseError> {
        use crate::schema::user_profiles::dsl;

        dsl::user_profiles
            .select((
                dsl::user_id,
                dsl::name,
                dsl::description,
                dsl::display_name,
                dsl::pfp,
                dsl::banner,
            ))
            .filter(dsl::user_id.eq(id.0))
            .load::<UserProfile>(&mut db.db)
            .map(|v| v.first().cloned())
            .map_err(DatabaseError::from)
    }
}

#[derive(AsChangeset, Serialize, Deserialize, JsonSchema, Debug, Default, Clone, PartialEq, Eq)]
#[diesel(table_name = crate::schema::user_profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserProfilePartial {
    pub name: Option<String>,
    pub description: Option<String>,
    pub display_name: Option<String>,
    pub pfp: Option<String>,
    pub banner: Option<String>,
}

impl UserProfilePartial {
    /// Partially update a club profile.
    /// fields that are `None` will have no affect on the database.
    pub fn unchecked_update_partial(
        &self,
        db: &mut DataState,
        user_id: &UserId,
    ) -> Result<(), DatabaseError> {
        use crate::schema::user_profiles::dsl;

        diesel::update(dsl::user_profiles)
            .set(self)
            .filter(dsl::user_id.eq(user_id.0))
            .execute(&mut db.db)
            .map(|_| ())
            .map_err(DatabaseError::from)
    }
}
