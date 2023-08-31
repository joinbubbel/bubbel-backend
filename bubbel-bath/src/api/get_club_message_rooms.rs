use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetClubMessageRooms {
    token: UserToken,
    club_id: ClubId,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetClubMessageRoomsOut {
    message_rooms: Vec<MessageRoomId>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum GetClubMessageRoomsError {
    NoAuth,
    NotClubMember,
    Internal { ierror: String },
}

pub async fn get_club_message_rooms(
    db: &mut DataStateInstance,
    auth: &AuthState,
    req: GetClubMessageRooms,
) -> Result<GetClubMessageRoomsOut, GetClubMessageRoomsError> {
    let user_id = auth
        .check_user_with_token(&req.token)
        .ok_or(GetClubMessageRoomsError::NoAuth)?;

    if !ClubMembers::is_user_in_club(db, &user_id, &req.club_id).map_err(|e| {
        GetClubMessageRoomsError::Internal {
            ierror: e.to_string(),
        }
    })? {
        Err(GetClubMessageRoomsError::NotClubMember)?;
    }

    let message_rooms = MessageRoom::get_club_message_rooms(db, &req.club_id).map_err(|e| {
        GetClubMessageRoomsError::Internal {
            ierror: e.to_string(),
        }
    })?;

    Ok(GetClubMessageRoomsOut { message_rooms })
}
