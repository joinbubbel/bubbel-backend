use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct AddFriendConnection {
    pub token: UserToken,
    pub receiver_id: UserId,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct AddFriendConnectionOut {}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum AddFriendConnectionError {
    NoAuth,
    CannotAddSelf,
    AlreadyConnected,
    Internal { ierror: String },
}

pub async fn add_friend_connection(
    db: &mut DataStateInstance,
    auth: &AuthState,
    req: AddFriendConnection,
) -> Result<AddFriendConnectionOut, AddFriendConnectionError> {
    let Some(user_id) = auth.check_user_with_token(&req.token) else {
        Err(AddFriendConnectionError::NoAuth)?
    };

    if user_id == req.receiver_id {
        Err(AddFriendConnectionError::CannotAddSelf)?;
    }

    if FriendConnection::has_already_added_user(db, &user_id, &req.receiver_id).map_err(|e| {
        AddFriendConnectionError::Internal {
            ierror: e.to_string(),
        }
    })? {
        Err(AddFriendConnectionError::AlreadyConnected)?;
    }

    FriendConnection::insert_new(db, &user_id, &req.receiver_id).map_err(|e| {
        AddFriendConnectionError::Internal {
            ierror: e.to_string(),
        }
    })?;

    Ok(AddFriendConnectionOut {})
}
