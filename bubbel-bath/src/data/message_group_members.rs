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
#[diesel(table_name = crate::schema::message_group_members)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MessageGroupMember {
    user_id: i32,
    message_group_id: i32,
}

impl MessageGroupMember {
    pub fn insert_new(
        db: &mut DataStateInstance,
        user_id: UserId,
        message_group_id: MessageGroupId,
    ) -> Result<(), DatabaseError> {
        use crate::schema::message_group_members::dsl;

        diesel::insert_into(dsl::message_group_members)
            .values(&Self {
                user_id: user_id.0,
                message_group_id: message_group_id.0,
            })
            .execute(&mut db.db)
            .map(|_| ())
            .map_err(DatabaseError::from)
    }

    pub fn get_message_group_memberships(
        db: &mut DataStateInstance,
        message_group_id: &MessageGroupId,
    ) -> Result<Vec<UserId>, DatabaseError> {
        use crate::schema::message_group_members::dsl;

        dsl::message_group_members
            .select(dsl::user_id)
            .filter(dsl::message_group_id.eq(message_group_id.0))
            .load::<i32>(&mut db.db)
            .map(|ids| ids.into_iter().map(UserId).collect())
            .map_err(DatabaseError::from)
    }

    pub fn is_user_in_message_group(
        db: &mut DataStateInstance,
        user_id: &UserId,
        message_group_id: &MessageGroupId,
    ) -> Result<bool, DatabaseError> {
        use crate::schema::message_group_members::dsl;

        Ok(dsl::message_group_members
            .select(dsl::message_group_id)
            .filter(dsl::user_id.eq(user_id.0))
            .filter(dsl::message_group_id.eq(message_group_id.0))
            .load::<i32>(&mut db.db)
            .map_err(DatabaseError::from)?
            .len()
            == 1)
    }

    pub fn get_user_message_groups(
        db: &mut DataStateInstance,
        user_id: &UserId,
    ) -> Result<Vec<MessageGroupId>, DatabaseError> {
        use crate::schema::message_group_members::dsl;

        dsl::message_group_members
            .select(dsl::message_group_id)
            .filter(dsl::user_id.eq(user_id.0))
            .load::<i32>(&mut db.db)
            .map(|ids| ids.into_iter().map(MessageGroupId).collect())
            .map_err(DatabaseError::from)
    }
}
