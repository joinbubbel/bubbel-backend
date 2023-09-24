use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct SetMessageGroupName {
    token: UserToken,
    message_group_id: MessageGroupId,
    name: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct SetMessageGroupNameOut {}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum SetMessageGroupNameError {
    NoAuth,
    NotGroupMember,
    PersonalDirectMessage,
    Internal { ierror: String },
}

pub async fn set_message_group_name(
    db: &mut DataStateInstance,
    auth: &AuthState,
    req: SetMessageGroupName,
) -> Result<SetMessageGroupNameOut, SetMessageGroupNameError> {
    let Some(user_id) = auth.check_user_with_token(&req.token) else {
        Err(SetMessageGroupNameError::NoAuth)?
    };
    let members = MessageGroupMember::get_message_group_memberships(db, &req.message_group_id)
        .map_err(|e| SetMessageGroupNameError::Internal {
            ierror: e.to_string(),
        })?;
    if members.len() >= 2 {
        Err(SetMessageGroupNameError::PersonalDirectMessage)?
    }
    if !members.contains(&user_id) {
        Err(SetMessageGroupNameError::NotGroupMember)?
    }

    MessageGroup::set_name(db, &req.message_group_id, req.name).map_err(|e| {
        SetMessageGroupNameError::Internal {
            ierror: e.to_string(),
        }
    })?;

    Ok(SetMessageGroupNameOut {})
}
