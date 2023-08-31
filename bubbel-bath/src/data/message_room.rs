use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct MessageRoomId(pub i32);

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
#[diesel(table_name = crate::schema::message_rooms)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MessageRoom {
    name: Option<String>,
    club_id: i32,
    dc_id: i32,
}

impl MessageRoom {
    pub fn insert_new(
        db: &mut DataStateInstance,
        name: String,
        club_id: ClubId,
        dc_id: DataChannelId,
    ) -> Result<MessageRoomId, DatabaseError> {
        use crate::schema::message_rooms::dsl;

        diesel::insert_into(dsl::message_rooms)
            .values(&MessageRoom {
                name: Some(name),
                club_id: club_id.0,
                dc_id: dc_id.0,
            })
            .returning(dsl::message_room_id)
            .get_result::<i32>(&mut db.db)
            .map(MessageRoomId)
            .map_err(DatabaseError::from)
    }

    pub fn get(
        db: &mut DataStateInstance,
        id: &MessageRoomId,
    ) -> Result<Option<Self>, DatabaseError> {
        use crate::schema::message_rooms::dsl;

        dsl::message_rooms
            .select(MessageRoom::as_select())
            .filter(dsl::message_room_id.eq(id.0))
            .load::<MessageRoom>(&mut db.db)
            .map(|v| v.first().cloned())
            .map_err(DatabaseError::from)
    }

    pub fn get_club_message_rooms(
        db: &mut DataStateInstance,
        club_id: &ClubId,
    ) -> Result<Vec<MessageRoomId>, DatabaseError> {
        use crate::schema::message_rooms::dsl;

        dsl::message_rooms
            .select(dsl::message_room_id)
            .filter(dsl::club_id.eq(club_id.0))
            .load::<i32>(&mut db.db)
            .map(|ids| ids.into_iter().map(MessageRoomId).collect())
            .map_err(DatabaseError::from)
    }
}
