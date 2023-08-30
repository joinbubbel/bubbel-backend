use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct RoomId(pub i32);

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
    ) -> Result<(), DatabaseError> {
        use crate::schema::message_rooms::dsl;

        diesel::insert_into(dsl::message_rooms)
            .values(&MessageRoom {
                name: Some(name),
                club_id: club_id.0,
                dc_id: dc_id.0,
            })
            .execute(&mut db.db)
            .map(|_| ())
            .map_err(DatabaseError::from)
    }
}
