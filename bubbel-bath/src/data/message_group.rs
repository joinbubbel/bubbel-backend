use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct MessageGroupId(pub i32);

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
#[diesel(table_name = crate::schema::message_groups)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MessageGroup {
    name: Option<String>,
    dc_id: i32,
}

impl MessageGroup {
    pub fn insert_new(
        db: &mut DataStateInstance,
        name: String,
        dc_id: DataChannelId,
    ) -> Result<MessageGroupId, DatabaseError> {
        use crate::schema::message_groups::dsl;

        diesel::insert_into(dsl::message_groups)
            .values(&MessageGroup {
                name: Some(name),
                dc_id: dc_id.0,
            })
            .returning(dsl::message_group_id)
            .get_result::<i32>(&mut db.db)
            .map(MessageGroupId)
            .map_err(DatabaseError::from)
    }

    pub fn get(
        db: &mut DataStateInstance,
        id: &MessageGroupId,
    ) -> Result<Option<Self>, DatabaseError> {
        use crate::schema::message_groups::dsl;

        dsl::message_groups
            .select(MessageGroup::as_select())
            .filter(dsl::message_group_id.eq(id.0))
            .load::<MessageGroup>(&mut db.db)
            .map(|v| v.first().cloned())
            .map_err(DatabaseError::from)
    }
}
