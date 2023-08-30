use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct UsernameToId {
    username: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct UsernameToIdOut {
    user_id: UserId,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum UsernameToIdError {
    UserNotFound,
    Internal { ierror: String },
}

pub async fn username_to_id(
    db: &mut DataStateInstance,
    req: UsernameToId,
) -> Result<UsernameToIdOut, UsernameToIdError> {
    let user_id = User::get_user_id_with_username(db, &req.username)
        .map_err(|e| UsernameToIdError::Internal {
            ierror: e.to_string(),
        })?
        .ok_or(UsernameToIdError::UserNotFound)?;

    Ok(UsernameToIdOut { user_id })
}
