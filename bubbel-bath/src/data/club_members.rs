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
    /// 1. Doesn't check if `user_id` is already in the club.
    pub fn insert_new(
        db: &mut DataStateInstance,
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

    /// Get all users associated with a club.
    pub fn get_club_memberships(
        db: &mut DataStateInstance,
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

    /// Get all clubs associated with a user.
    pub fn get_user_clubs(
        db: &mut DataStateInstance,
        user_id: &UserId,
    ) -> Result<Vec<ClubId>, DatabaseError> {
        use crate::schema::club_members::dsl;

        dsl::club_members
            .select(dsl::club_id)
            .filter(dsl::user_id.eq(user_id.0))
            .load::<i32>(&mut db.db)
            .map(|ids| ids.into_iter().map(ClubId).collect())
            .map_err(DatabaseError::from)
    }

    pub fn is_user_in_club(
        db: &mut DataStateInstance,
        user_id: &UserId,
        club_id: &ClubId,
    ) -> Result<bool, DatabaseError> {
        use crate::schema::club_members::dsl;

        Ok(dsl::club_members
            .select(dsl::club_id)
            .filter(dsl::user_id.eq(user_id.0))
            .filter(dsl::club_id.eq(club_id.0))
            .load::<i32>(&mut db.db)
            .map_err(DatabaseError::from)?
            .len()
            == 1)
    }

    /// Remove a club connection.
    /// Nothing happens if the club doesn't exist.
    pub fn remove(
        db: &mut DataStateInstance,
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
