use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetMessageGroupMembers {
    token: UserToken,
    message_group_id: MessageGroupId,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetMessageGroupMembersOut {
    members: Vec<UserId>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum GetMessageGroupMembersError {
    NoAuth,
    NotGroupMember,
    Internal { ierror: String },
}

pub async fn get_message_group_members(
    db: &mut DataStateInstance,
    auth: &AuthState,
    req: GetMessageGroupMembers,
) -> Result<GetMessageGroupMembersOut, GetMessageGroupMembersError> {
    let Some(user_id) = auth.check_user_with_token(&req.token) else {
        return Err(GetMessageGroupMembersError::NoAuth)?;
    };

    if !MessageGroupMember::is_user_in_message_group(db, &user_id, &req.message_group_id).map_err(
        |e| GetMessageGroupMembersError::Internal {
            ierror: e.to_string(),
        },
    )? {
        return Err(GetMessageGroupMembersError::NotGroupMember)?;
    }

    let members = MessageGroupMember::get_message_group_memberships(db, &req.message_group_id)
        .map_err(|e| GetMessageGroupMembersError::Internal {
            ierror: e.to_string(),
        })?;

    Ok(GetMessageGroupMembersOut { members })
}
