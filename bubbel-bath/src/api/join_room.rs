use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct JoinRoom {
    pub token: UserToken,
    pub club_id: ClubId,
    pub message_room_id: MessageRoomId,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct JoinRoomOut {}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum JoinRoomError {
    NoAuth,
    NotClubMember,
    AlreadyJoined,
    Internal { ierror: String },
}

pub fn join_room(
    db: &mut DataStateInstance,
    auth: &AuthState,
    req: JoinRoom,
) -> Result<JoinRoomOut, JoinRoomError> {
    let Some(user_id) = auth.check_user_with_token(&req.token) else {
        Err(JoinRoomError::NoAuth)?
    };

    if !ClubMembers::is_user_in_club(db, &user_id, &req.club_id).map_err(|e| {
        JoinRoomError::Internal {
            ierror: e.to_string(),
        }
    })? {
        Err(JoinRoomError::NotClubMember)?
    }

    if MessageRoomMember::is_user_in_message_room(db, &user_id, &req.message_room_id).map_err(
        |e| JoinRoomError::Internal {
            ierror: e.to_string(),
        },
    )? {
        Err(JoinRoomError::AlreadyJoined)?
    }

    MessageRoomMember::insert_new(db, user_id, req.message_room_id).map_err(|e| {
        JoinRoomError::Internal {
            ierror: e.to_string(),
        }
    })?;

    Ok(JoinRoomOut {})
}
