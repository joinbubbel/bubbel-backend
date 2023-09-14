use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetMessageRoom {
    token: UserToken,
    message_room_id: MessageRoomId,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetMessageRoomOut {
    #[serde(flatten)]
    message_room: MessageRoom,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum GetMessageRoomError {
    NoAuth,
    NotMessageRoomMember,
    MessageRoomNotFound,
    Internal { ierror: String },
}

pub async fn get_message_room(
    db: &mut DataStateInstance,
    auth: &AuthState,
    req: GetMessageRoom,
) -> Result<GetMessageRoomOut, GetMessageRoomError> {
    let user_id = auth
        .check_user_with_token(&req.token)
        .ok_or(GetMessageRoomError::NoAuth)?;

    if !MessageRoomMember::is_user_in_message_room(db, &user_id, &req.message_room_id).map_err(
        |e| GetMessageRoomError::Internal {
            ierror: e.to_string(),
        },
    )? {
        // Err(GetMessageRoomError::NotMessageRoomMember)?;
    }

    let message_room = MessageRoom::get(db, &req.message_room_id)
        .map_err(|e| GetMessageRoomError::Internal {
            ierror: e.to_string(),
        })?
        .ok_or(GetMessageRoomError::MessageRoomNotFound)?;

    Ok(GetMessageRoomOut { message_room })
}
