use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct CreateMessageGroup {
    token: UserToken,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct CreateMessageGroupOut {
    message_group_id: MessageGroupId,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum CreateMessageGroupError {
    NoAuth,
    Internal { ierror: String },
}

pub async fn create_message_group(
    db: &mut DataStateInstance,
    auth: &AuthState,
    req: CreateMessageGroup,
) -> Result<CreateMessageGroupOut, CreateMessageGroupError> {
    let user_id = auth
        .check_user_with_token(&req.token)
        .ok_or(CreateMessageGroupError::NoAuth)?;

    let (_, dc_id) =
        DataChannel::insert_new(db).map_err(|e| CreateMessageGroupError::Internal {
            ierror: e.to_string(),
        })?;

    let message_group_id = MessageGroup::insert_new(db, None, dc_id).map_err(|e| {
        CreateMessageGroupError::Internal {
            ierror: e.to_string(),
        }
    })?;

    MessageGroupMember::insert_new(db, user_id, message_group_id).map_err(|e| {
        CreateMessageGroupError::Internal {
            ierror: e.to_string(),
        }
    })?;

    Ok(CreateMessageGroupOut { message_group_id })
}
