use super::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetFriendConnections {
    pub token: UserToken,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetFriendConnectionsOut {
    pub friend_connections: HashMap<UserId, FriendStatus>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum GetFriendConnectionsError {
    NoAuth,
    Internal { ierror: String },
}

pub async fn get_friend_connections(
    db: &mut DataStateInstance,
    auth: &AuthState,
    req: GetFriendConnections,
) -> Result<GetFriendConnectionsOut, GetFriendConnectionsError> {
    let Some(user_id) = auth.check_user_with_token(&req.token) else {
        Err(GetFriendConnectionsError::NoAuth)?
    };

    let friend_connections =
        FriendConnection::get_friend_connections(db, &user_id).map_err(|e| {
            GetFriendConnectionsError::Internal {
                ierror: e.to_string(),
            }
        })?;

    Ok(GetFriendConnectionsOut { friend_connections })
}
