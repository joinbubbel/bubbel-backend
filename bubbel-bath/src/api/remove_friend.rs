use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct RemoveFriend {
    pub token: UserToken,
    pub removal_id: UserId,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct RemoveFriendOut {}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum RemoveFriendError {
    NoAuth,
    Internal { ierror: String },
}

pub async fn remove_friend(
    db: &mut DataStateInstance,
    auth: &AuthState,
    req: RemoveFriend,
) -> Result<RemoveFriendOut, RemoveFriendError> {
    let Some(user_id) = auth.check_user_with_token(&req.token) else {
        Err(RemoveFriendError::NoAuth)?
    };

    FriendConnection::remove(db, &user_id, &req.removal_id).map_err(|e| {
        RemoveFriendError::Internal {
            ierror: e.to_string(),
        }
    })?;

    Ok(RemoveFriendOut {})
}
