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
#[diesel(table_name = crate::schema::message_room_members)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MessageRoomMember {
    user_id: i32,
    room_id: i32,
}

impl MessageRoomMember {
    pub fn insert_new(
        db: &mut DataStateInstance,
        user_id: UserId,
        room_id: MessageRoomId,
    ) -> Result<(), DatabaseError> {
        use crate::schema::message_room_members::dsl;

        diesel::insert_into(dsl::message_room_members)
            .values(&MessageRoomMember {
                user_id: user_id.0,
                room_id: room_id.0,
            })
            .execute(&mut db.db)
            .map(|_| ())
            .map_err(DatabaseError::from)
    }

    pub fn is_user_in_message_room(
        db: &mut DataStateInstance,
        user_id: &UserId,
        club_id: &MessageRoomId,
    ) -> Result<bool, DatabaseError> {
        use crate::schema::message_room_members::dsl;

        Ok(dsl::message_room_members
            .select(dsl::room_id)
            .filter(dsl::user_id.eq(user_id.0))
            .filter(dsl::room_id.eq(club_id.0))
            .load::<i32>(&mut db.db)
            .map_err(DatabaseError::from)?
            .len()
            == 1)
    }
}
