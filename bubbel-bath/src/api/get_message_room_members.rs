use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetMessageRoomMembers {
    token: UserToken,
    message_room_id: MessageRoomId,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetMessageRoomMembersOut {
    members: Vec<UserId>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum GetMessageRoomMembersError {
    NoAuth,
    NotMessageRoomMember,
    Internal { ierror: String },
}

pub async fn get_message_room_members(
    db: &mut DataStateInstance,
    auth: &AuthState,
    req: GetMessageRoomMembers,
) -> Result<GetMessageRoomMembersOut, GetMessageRoomMembersError> {
    let user_id = auth
        .check_user_with_token(&req.token)
        .ok_or(GetMessageRoomMembersError::NoAuth)?;

    let members = MessageRoomMember::get_message_room_memberships(db, &req.message_room_id)
        .map_err(|e| GetMessageRoomMembersError::Internal {
            ierror: e.to_string(),
        })?;

    if !members.iter().any(|&member| member == user_id) {
        Err(GetMessageRoomMembersError::NotMessageRoomMember)?;
    }

    Ok(GetMessageRoomMembersOut { members })
}
