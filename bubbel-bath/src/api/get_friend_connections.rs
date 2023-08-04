use super::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetFriendConnection {
    pub token: UserToken,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetFriendConnectionOut {
    pub friend_connections: HashMap<UserId, FriendStatus>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum GetFriendConnectionError {
    NoAuth,
    Internal { ierror: String },
}

pub fn get_friend_connections(
    db: &mut DataState,
    auth: &AuthState,
    req: GetFriendConnection,
) -> Result<GetFriendConnectionOut, GetFriendConnectionError> {
    let Some(user_id) = auth.check_user_with_token(&req.token) else {
        Err(GetFriendConnectionError::NoAuth)?
    };

    let friend_connections =
        FriendConnection::get_friend_connections(db, &user_id).map_err(|e| {
            GetFriendConnectionError::Internal {
                ierror: e.to_string(),
            }
        })?;

    Ok(GetFriendConnectionOut { friend_connections })
}
