use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct CreateClub {
    pub token: UserToken,
    pub name: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct CreateClubOut {
    pub club_id: ClubId,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum CreateClubError {
    NoAuth,
    ClubAlreadyExists,
    Internal { ierror: String },
}

pub async fn create_club(
    db: &mut DataStateInstance,
    auth: &AuthState,
    req: CreateClub,
) -> Result<CreateClubOut, CreateClubError> {
    let Some(user_id) = auth.check_user_with_token(&req.token) else {
        Err(CreateClubError::NoAuth)?
    };

    let (_, dc_id) = DataChannel::insert_new(db).map_err(|e| CreateClubError::Internal {
        ierror: e.to_string(),
    })?;

    let club_id = ClubProfile::insert_new(db, &user_id, &dc_id, req.name).map_err(|e| match e {
        DatabaseError::UniqueViolation => CreateClubError::ClubAlreadyExists,
        e => CreateClubError::Internal {
            ierror: e.to_string(),
        },
    })?;

    join_club(
        db,
        auth,
        JoinClub {
            token: req.token.clone(),
            club_id,
        },
    )
    .await
    .map_err(|e| match e {
        JoinClubError::NoAuth => CreateClubError::Internal {
            ierror: "Reached impossible token case.".to_owned(),
        },
        JoinClubError::AlreadyJoined => CreateClubError::Internal {
            ierror: "Reached impossible already joined case".to_owned(),
        },
        JoinClubError::Internal { ierror } => CreateClubError::Internal { ierror },
    })?;

    Ok(CreateClubOut { club_id })
}
