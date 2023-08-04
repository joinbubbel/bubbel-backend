use super::*;
use std::collections::HashMap;

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
#[diesel(table_name = crate::schema::friend_connections)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FriendConnection {
    send_user_id: i32,
    recv_user_id: i32,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub enum FriendStatus {
    SentPending,
    RecievedPending,
    Full,
}

impl FriendConnection {
    /// Create a new friend connection between a sender and reciever.
    /// 1. Does not check if you are friending yourself.
    /// 2. Does not check if this connection already exists.
    pub fn insert_new(
        db: &mut DataState,
        send_user_id: &UserId,
        recv_user_id: &UserId,
    ) -> Result<(), DatabaseError> {
        use crate::schema::friend_connections::dsl;

        let already_exists = dsl::friend_connections
            .select(FriendConnection::as_select())
            .filter(dsl::send_user_id.eq(send_user_id.0))
            .filter(dsl::send_user_id.eq(recv_user_id.0))
            .load::<FriendConnection>(&mut db.db)
            .map_err(DatabaseError::from)?
            .len()
            == 1;

        if already_exists {
            return Ok(());
        }

        diesel::insert_into(dsl::friend_connections)
            .values(&FriendConnection {
                send_user_id: send_user_id.0,
                recv_user_id: recv_user_id.0,
            })
            .execute(&mut db.db)
            .map(|_| ())
            .map_err(DatabaseError::from)
    }

    /// Checks if `send_user_id` has already connected with `recv_user_id`.
    pub fn has_already_added_user(
        db: &mut DataState,
        send_user_id: &UserId,
        recv_user_id: &UserId,
    ) -> Result<bool, DatabaseError> {
        use crate::schema::friend_connections::dsl;

        Ok(dsl::friend_connections
            .select(FriendConnection::as_select())
            .filter(dsl::send_user_id.eq(send_user_id.0))
            .filter(dsl::recv_user_id.eq(recv_user_id.0))
            .load::<FriendConnection>(&mut db.db)
            .map_err(DatabaseError::from)?
            .len()
            == 1)
    }

    /// The all friend connection statuses.
    pub fn get_friend_connections(
        db: &mut DataState,
        send_user_id: &UserId,
    ) -> Result<HashMap<UserId, FriendStatus>, DatabaseError> {
        use crate::schema::friend_connections::dsl;

        let senders = dsl::friend_connections
            .select(FriendConnection::as_select())
            .filter(dsl::send_user_id.eq(send_user_id.0))
            .load::<FriendConnection>(&mut db.db)
            .map_err(DatabaseError::from)?;

        let recvers = dsl::friend_connections
            .select(FriendConnection::as_select())
            .filter(dsl::recv_user_id.eq(send_user_id.0))
            .load::<FriendConnection>(&mut db.db)
            .map_err(DatabaseError::from)?;

        let mut ret = HashMap::new();

        for sender in senders.iter() {
            ret.entry(UserId(sender.recv_user_id))
                .or_insert(FriendStatus::SentPending);

            for recver in recvers.iter() {
                ret.entry(UserId(recver.send_user_id))
                    .or_insert(FriendStatus::RecievedPending);

                if sender.send_user_id == recver.recv_user_id {
                    ret.insert(UserId(sender.recv_user_id), FriendStatus::Full);
                } else {
                }
            }
        }

        Ok(ret)
    }

    /// Completely remove a friend connection both ways.
    pub fn remove(
        db: &mut DataState,
        send_user_id: &UserId,
        recv_user_id: &UserId,
    ) -> Result<(), DatabaseError> {
        use crate::schema::friend_connections::dsl;

        diesel::delete(dsl::friend_connections)
            .filter(dsl::send_user_id.eq(send_user_id.0))
            .filter(dsl::recv_user_id.eq(recv_user_id.0))
            .execute(&mut db.db)
            .map(|_| ())
            .map_err(DatabaseError::from)?;
        diesel::delete(dsl::friend_connections)
            .filter(dsl::send_user_id.eq(recv_user_id.0))
            .filter(dsl::recv_user_id.eq(send_user_id.0))
            .execute(&mut db.db)
            .map(|_| ())
            .map_err(DatabaseError::from)?;
        Ok(())
    }
}
