use super::*;

#[derive(
    Queryable,
    Selectable,
    Insertable,
    Serialize,
    Deserialize,
    JsonSchema,
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
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
    pub fn insert_new(
        db: &mut DataStateInstance,
        user_id: &UserId,
        name: Option<String>,
        display_name: Option<String>,
    ) -> Result<(), DatabaseError> {
        use crate::schema::user_profiles::dsl;

        diesel::insert_into(dsl::user_profiles)
            .values(&UserProfile {
                user_id: user_id.0,
                name,
                description: None,
                display_name,
                pfp: None,
                banner: None,
            })
            .execute(&mut db.db)
            .map(|_| ())
            .map_err(DatabaseError::from)
    }

    /// Try to get a user profile using `id`.
    pub fn get(
        db: &mut DataStateInstance,
        id: UserId,
    ) -> Result<Option<UserProfile>, DatabaseError> {
        use crate::schema::user_profiles::dsl;

        dsl::user_profiles
            .select(UserProfile::as_select())
            .filter(dsl::user_id.eq(id.0))
            .load::<UserProfile>(&mut db.db)
            .map(|v| v.first().cloned())
            .map_err(DatabaseError::from)
    }

    /// Of all the users, get `batch_index` of `batch_size` of user ids and display names.
    /// The result is in the order of `user_id`.
    pub fn get_ordered_batch(
        db: &mut DataStateInstance,
        batch_index: usize,
        batch_size: usize,
    ) -> Result<Vec<(UserId, String)>, DatabaseError> {
        use crate::schema::user_profiles::dsl;

        dsl::user_profiles
            .select((dsl::user_id, dsl::display_name))
            .order_by(dsl::user_id.asc())
            .offset((batch_size * batch_index) as i64)
            .limit(batch_size as i64)
            .load::<(i32, Option<String>)>(&mut db.db)
            .map(|v| {
                v.into_iter()
                    .filter(|(_, name)| name.is_some())
                    .map(|(id, name)| (UserId(id), name.unwrap()))
                    .collect::<Vec<_>>()
            })
            .map_err(DatabaseError::from)
    }

    /// TODO Temporary.
    pub fn get_random(db: &mut DataStateInstance) -> Result<Vec<(UserId, Self)>, DatabaseError> {
        use crate::schema::user_profiles::dsl;

        dsl::user_profiles
            .select((dsl::user_id, UserProfile::as_select()))
            .limit(30)
            .load::<(i32, UserProfile)>(&mut db.db)
            .map(|v| {
                v.into_iter()
                    .map(|(id, profile)| (UserId(id), profile))
                    .collect::<Vec<_>>()
            })
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
        db: &mut DataStateInstance,
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
