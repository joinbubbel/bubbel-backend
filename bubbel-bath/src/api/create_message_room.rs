use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct CreateMessageRoom {
    token: UserToken,
    club_id: ClubId,
    name: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct CreateMessageRoomOut {
    message_room_id: MessageRoomId,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum CreateMessageRoomError {
    NoAuth,
    NotOwner,
    ClubNotFound,
    Internal { ierror: String },
}

pub async fn create_message_room(
    db: &mut DataStateInstance,
    auth: &AuthState,
    req: CreateMessageRoom,
) -> Result<CreateMessageRoomOut, CreateMessageRoomError> {
    let Some(user_id) = auth.check_user_with_token(&req.token) else {
        Err(CreateMessageRoomError::NoAuth)?
    };

    let (_, dc_id) = DataChannel::insert_new(db).map_err(|e| CreateMessageRoomError::Internal {
        ierror: e.to_string(),
    })?;

    let club_profile = ClubProfile::get(db, &req.club_id)
        .map_err(|e| CreateMessageRoomError::Internal {
            ierror: e.to_string(),
        })?
        .ok_or(CreateMessageRoomError::ClubNotFound)?;

    if UserId(club_profile.owner) != user_id {
        Err(CreateMessageRoomError::NotOwner)?;
    }

    let message_room_id =
        MessageRoom::insert_new(db, req.name, req.club_id, dc_id).map_err(|e| {
            CreateMessageRoomError::Internal {
                ierror: e.to_string(),
            }
        })?;

    join_message_room(
        db,
        auth,
        JoinMessageRoom {
            token: req.token,
            club_id: req.club_id,
            message_room_id,
        },
    )
    .await
    .map_err(|e| match e {
        JoinMessageRoomError::Internal { ierror } => CreateMessageRoomError::Internal { ierror },
        e => CreateMessageRoomError::Internal {
            ierror: format!("Reached impossible case: {:?}.", e),
        },
    })?;

    Ok(CreateMessageRoomOut { message_room_id })
}
