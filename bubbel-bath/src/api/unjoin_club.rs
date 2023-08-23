use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct UnjoinClub {
    pub token: UserToken,
    pub club_id: ClubId,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct UnjoinClubOut {}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum UnjoinClubError {
    NoAuth,
    ClubNotFound,
    CannotUnjoinAsOwner,
    Internal { ierror: String },
}

pub fn unjoin_club(
    db: &mut DataStateInstance,
    auth: &AuthState,
    req: UnjoinClub,
) -> Result<UnjoinClubOut, UnjoinClubError> {
    let Some(user_id) = auth.check_user_with_token(&req.token) else {
        Err(UnjoinClubError::NoAuth)?
    };

    let Some(club_profile) =
        ClubProfile::get(db, &req.club_id).map_err(|e| UnjoinClubError::Internal {
            ierror: e.to_string(),
        })?
    else {
        Err(UnjoinClubError::ClubNotFound)?
    };

    if club_profile.owner == user_id.0 {
        Err(UnjoinClubError::CannotUnjoinAsOwner)?;
    }

    ClubMembers::remove(db, &user_id, &req.club_id).map_err(|e| UnjoinClubError::Internal {
        ierror: e.to_string(),
    })?;
    Ok(UnjoinClubOut {})
}
