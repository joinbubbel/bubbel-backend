use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetMessageGroup {
    token: UserToken,
    message_group_id: MessageGroupId,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetMessageGroupOut {
    #[serde(flatten)]
    group: MessageGroup,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum GetMessageGroupError {
    NoAuth,
    GroupNotFound,
    NotGroupMember,
    Internal { ierror: String },
}

pub async fn get_message_group(
    db: &mut DataStateInstance,
    auth: &AuthState,
    req: GetMessageGroup,
) -> Result<GetMessageGroupOut, GetMessageGroupError> {
    let Some(user_id) = auth.check_user_with_token(&req.token) else {
        return Err(GetMessageGroupError::NoAuth)?;
    };

    if !MessageGroupMember::is_user_in_message_group(db, &user_id, &req.message_group_id).map_err(
        |e| GetMessageGroupError::Internal {
            ierror: e.to_string(),
        },
    )? {
        return Err(GetMessageGroupError::NotGroupMember)?;
    }

    let group = MessageGroup::get(db, &req.message_group_id)
        .map_err(|e| GetMessageGroupError::Internal {
            ierror: e.to_string(),
        })?
        .ok_or(GetMessageGroupError::GroupNotFound)?;

    Ok(GetMessageGroupOut { group })
}
