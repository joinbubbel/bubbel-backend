use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct JoinMessageRoom {
    pub token: UserToken,
    pub club_id: ClubId,
    pub message_room_id: MessageRoomId,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct JoinMessageRoomOut {}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum JoinMessageRoomError {
    NoAuth,
    NotClubMember,
    AlreadyJoined,
    Internal { ierror: String },
}

pub async fn join_message_room(
    db: &mut DataStateInstance,
    auth: &AuthState,
    req: JoinMessageRoom,
) -> Result<JoinMessageRoomOut, JoinMessageRoomError> {
    let Some(user_id) = auth.check_user_with_token(&req.token) else {
        Err(JoinMessageRoomError::NoAuth)?
    };

    if !ClubMembers::is_user_in_club(db, &user_id, &req.club_id).map_err(|e| {
        JoinMessageRoomError::Internal {
            ierror: e.to_string(),
        }
    })? {
        Err(JoinMessageRoomError::NotClubMember)?
    }

    if MessageRoomMember::is_user_in_message_room(db, &user_id, &req.message_room_id).map_err(
        |e| JoinMessageRoomError::Internal {
            ierror: e.to_string(),
        },
    )? {
        Err(JoinMessageRoomError::AlreadyJoined)?
    }

    MessageRoomMember::insert_new(db, user_id, req.message_room_id).map_err(|e| {
        JoinMessageRoomError::Internal {
            ierror: e.to_string(),
        }
    })?;

    Ok(JoinMessageRoomOut {})
}
