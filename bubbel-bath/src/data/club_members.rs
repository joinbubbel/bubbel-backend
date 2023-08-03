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
#[diesel(table_name = crate::schema::club_members)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ClubMembers {
    user_id: i32,
    club_id: i32,
}

impl ClubMembers {
    /// Inserts an club membership with `user_id` and `club_id`.
    pub fn insert_new(
        db: &mut DataState,
        user_id: &UserId,
        club_id: &ClubId,
    ) -> Result<(), DatabaseError> {
        use crate::schema::club_members::dsl;

        diesel::insert_into(dsl::club_members)
            .values(&ClubMembers {
                user_id: user_id.0,
                club_id: club_id.0,
            })
            .execute(&mut db.db)
            .map(|_| ())
            .map_err(DatabaseError::from)
    }

    pub fn get_club_memberships(
        db: &mut DataState,
        club_id: &ClubId,
    ) -> Result<Vec<UserId>, DatabaseError> {
        use crate::schema::club_members::dsl;

        dsl::club_members
            .select(dsl::user_id)
            .filter(dsl::club_id.eq(club_id.0))
            .load::<i32>(&mut db.db)
            .map(|ids| ids.into_iter().map(UserId).collect())
            .map_err(DatabaseError::from)
    }

    /// Remove a club connection.
    /// Nothing happens if the club doesn't exist.
    pub fn remove(
        db: &mut DataState,
        user_id: &UserId,
        club_id: &ClubId,
    ) -> Result<(), DatabaseError> {
        use crate::schema::club_members::dsl;

        diesel::delete(dsl::club_members)
            .filter(dsl::user_id.eq(user_id.0))
            .filter(dsl::club_id.eq(club_id.0))
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
    /// Partially update a club profile.
    /// fields that are `None` will have no affect on the database.
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
