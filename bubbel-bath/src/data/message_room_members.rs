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
        room_id: RoomId,
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
}
