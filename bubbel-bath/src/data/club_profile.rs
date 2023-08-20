use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ClubId(pub i32);

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
#[diesel(table_name = crate::schema::club_profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ClubProfile {
    pub owner: i32,
    pub name: String,
    pub dc_id: i32,
    pub description: Option<String>,
    pub display_name: Option<String>,
    pub pfp: Option<String>,
    pub banner: Option<String>,
}

impl ClubProfile {
    /// Inserts an empty club profile with `owner_id` and `name`.
    pub fn insert_new(
        db: &mut DataStateInstance,
        owner_id: &UserId,
        dc_id: &DataChannelId,
        name: String,
    ) -> Result<ClubId, DatabaseError> {
        use crate::schema::club_profiles::dsl;

        diesel::insert_into(dsl::club_profiles)
            .values(&ClubProfile {
                owner: owner_id.0,
                name: name.clone(),
                dc_id: dc_id.0,
                description: None,
                display_name: Some(name),
                pfp: None,
                banner: None,
            })
            .returning(dsl::id)
            .get_result::<i32>(&mut db.db)
            .map(ClubId)
            .map_err(DatabaseError::from)
    }

    /// Try to get a club profile using `id`.
    pub fn get(db: &mut DataStateInstance, id: &ClubId) -> Result<Option<Self>, DatabaseError> {
        use crate::schema::club_profiles::dsl;

        dsl::club_profiles
            .select(ClubProfile::as_select())
            .filter(dsl::id.eq(id.0))
            .load::<ClubProfile>(&mut db.db)
            .map(|v| v.first().cloned())
            .map_err(DatabaseError::from)
    }

    /// Try to get a club using `name`.
    pub fn get_club_id_with_name(
        db: &mut DataStateInstance,
        name: &str,
    ) -> Result<Option<ClubId>, DatabaseError> {
        use crate::schema::club_profiles::dsl;

        dsl::club_profiles
            .select(dsl::id)
            .filter(dsl::name.eq(name))
            .load::<i32>(&mut db.db)
            .map(|u| u.first().map(|&u| ClubId(u)))
            .map_err(DatabaseError::from)
    }

    /// TODO Temporary.
    pub fn get_random(db: &mut DataStateInstance) -> Result<Vec<(ClubId, Self)>, DatabaseError> {
        use crate::schema::club_profiles::dsl;

        dsl::club_profiles
            .select((dsl::id, ClubProfile::as_select()))
            .limit(30)
            .load::<(i32, ClubProfile)>(&mut db.db)
            .map(|v| {
                v.into_iter()
                    .map(|(id, profile)| (ClubId(id), profile))
                    .collect::<Vec<_>>()
            })
            .map_err(DatabaseError::from)
    }

    /// Of all the clubs, get `batch_index` of `batch_size` of club ids and display names.
    /// The result is in the order of `club_id`.
    pub fn get_ordered_batch(
        db: &mut DataStateInstance,
        batch_index: usize,
        batch_size: usize,
    ) -> Result<Vec<(ClubId, String)>, DatabaseError> {
        use crate::schema::club_profiles::dsl;

        dsl::club_profiles
            .select((dsl::id, dsl::display_name))
            .order_by(dsl::id.asc())
            .offset((batch_size * batch_index) as i64)
            .limit(batch_size as i64)
            .load::<(i32, Option<String>)>(&mut db.db)
            .map(|v| {
                v.into_iter()
                    .filter(|(_, name)| name.is_some())
                    .map(|(id, name)| (ClubId(id), name.unwrap()))
                    .collect::<Vec<_>>()
            })
            .map_err(DatabaseError::from)
    }

    /// Remove a club profile using `id`.
    /// Nothing happens if the club doesn't exist.
    pub fn remove(db: &mut DataStateInstance, id: ClubId) -> Result<(), DatabaseError> {
        use crate::schema::club_profiles::dsl;

        diesel::delete(dsl::club_profiles)
            .filter(dsl::id.eq(id.0))
            .execute(&mut db.db)
            .map(|_| ())
            .map_err(DatabaseError::from)
    }
}

#[derive(AsChangeset, Serialize, Deserialize, JsonSchema, Debug, Default, Clone, PartialEq, Eq)]
#[diesel(table_name = crate::schema::club_profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ClubProfilePartial {
    pub owner: Option<i32>,
    pub name: Option<String>,
    pub dc_id: Option<i32>,
    pub description: Option<String>,
    pub display_name: Option<String>,
    pub pfp: Option<String>,
    pub banner: Option<String>,
}

impl ClubProfilePartial {
    /// Partially update a club profile.
    /// fields that are `None` will have no affect on the database.
    pub fn unchecked_partial_update(
        &self,
        db: &mut DataStateInstance,
        club_id: &ClubId,
    ) -> Result<(), DatabaseError> {
        use crate::schema::club_profiles::dsl;

        diesel::update(dsl::club_profiles)
            .set(self)
            .filter(dsl::id.eq(club_id.0))
            .execute(&mut db.db)
            .map(|_| ())
            .map_err(DatabaseError::from)
    }
}
