use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct CreateRoom {
    token: UserToken,
    club_id: ClubId,
    name: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct CreateRoomOut {
    message_room_id: MessageRoomId,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum CreateRoomError {
    NoAuth,
    NotOwner,
    ClubNotFound,
    Internal { ierror: String },
}

pub fn create_room(
    db: &mut DataStateInstance,
    auth: &AuthState,
    req: CreateRoom,
) -> Result<CreateRoomOut, CreateRoomError> {
    let Some(user_id) = auth.check_user_with_token(&req.token) else {
        Err(CreateRoomError::NoAuth)?
    };

    let (_, dc_id) = DataChannel::insert_new(db).map_err(|e| CreateRoomError::Internal {
        ierror: e.to_string(),
    })?;

    let club_profile = ClubProfile::get(db, &req.club_id)
        .map_err(|e| CreateRoomError::Internal {
            ierror: e.to_string(),
        })?
        .ok_or(CreateRoomError::ClubNotFound)?;

    if UserId(club_profile.owner) != user_id {
        Err(CreateRoomError::NotOwner)?;
    }

    let message_room_id =
        MessageRoom::insert_new(db, req.name, req.club_id, dc_id).map_err(|e| {
            CreateRoomError::Internal {
                ierror: e.to_string(),
            }
        })?;

    join_room(
        db,
        auth,
        JoinRoom {
            token: req.token,
            club_id: req.club_id,
            message_room_id,
        },
    )
    .map_err(|e| match e {
        JoinRoomError::Internal { ierror } => CreateRoomError::Internal { ierror },
        e => CreateRoomError::Internal {
            ierror: format!("Reached impossible case: {:?}.", e),
        },
    })?;

    Ok(CreateRoomOut { message_room_id })
}
