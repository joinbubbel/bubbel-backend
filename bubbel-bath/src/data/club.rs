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
    pub description: Option<String>,
    pub display_name: Option<String>,
    pub pfp: Option<String>,
    pub banner: Option<String>,
}

impl ClubProfile {
    pub fn insert_new(
        db: &mut DataState,
        owner_id: &UserId,
        name: String,
    ) -> Result<ClubId, DatabaseError> {
        use crate::schema::club_profiles::dsl;

        diesel::insert_into(dsl::club_profiles)
            .values(&ClubProfile {
                owner: owner_id.0,
                name,
                description: None,
                display_name: None,
                pfp: None,
                banner: None,
            })
            .returning(dsl::id)
            .get_result::<i32>(&mut db.db)
            .map(ClubId)
            .map_err(DatabaseError::from)
    }

    pub fn get(db: &mut DataState, id: &ClubId) -> Result<Option<Self>, DatabaseError> {
        use crate::schema::club_profiles::dsl;

        dsl::club_profiles
            .select((
                dsl::owner,
                dsl::name,
                dsl::description,
                dsl::display_name,
                dsl::pfp,
                dsl::banner,
            ))
            .filter(dsl::id.eq(id.0))
            .load::<ClubProfile>(&mut db.db)
            .map(|v| v.first().cloned())
            .map_err(DatabaseError::from)
    }

    pub fn remove(db: &mut DataState, id: ClubId) -> Result<(), DatabaseError> {
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
    pub description: Option<String>,
    pub display_name: Option<String>,
    pub pfp: Option<String>,
    pub banner: Option<String>,
}

impl ClubProfilePartial {
    pub fn unchecked_partial_update(
        &self,
        db: &mut DataState,
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
