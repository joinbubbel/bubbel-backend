use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct AddUserToMessageGroup {
    token: UserToken,
    add_user_id: UserId,
    message_group_id: MessageGroupId,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct AddUserToMessageGroupOut {}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum AddUserToMessageGroupError {
    NoAuth,
    NotGroupMember,
    AlreadyInGroup,
    Internal { ierror: String },
}

pub async fn add_user_to_message_group(
    db: &mut DataStateInstance,
    auth: &AuthState,
    req: AddUserToMessageGroup,
) -> Result<AddUserToMessageGroupOut, AddUserToMessageGroupError> {
    let user_id = auth
        .check_user_with_token(&req.token)
        .ok_or(AddUserToMessageGroupError::NoAuth)?;

    if MessageGroupMember::is_user_in_message_group(db, &req.add_user_id, &req.message_group_id)
        .map_err(|e| AddUserToMessageGroupError::Internal {
            ierror: e.to_string(),
        })?
    {
        Err(AddUserToMessageGroupError::AlreadyInGroup)?;
    }

    if !MessageGroupMember::is_user_in_message_group(db, &user_id, &req.message_group_id).map_err(
        |e| AddUserToMessageGroupError::Internal {
            ierror: e.to_string(),
        },
    )? {
        Err(AddUserToMessageGroupError::NotGroupMember)?;
    }

    MessageGroupMember::insert_new(db, req.add_user_id, req.message_group_id).map_err(|e| {
        AddUserToMessageGroupError::Internal {
            ierror: e.to_string(),
        }
    })?;

    Ok(AddUserToMessageGroupOut {})
}
