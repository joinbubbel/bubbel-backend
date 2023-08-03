use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct UnjoinClub {
    token: UserToken,
    club_id: ClubId,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct UnjoinClubOut {}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum UnjoinClubError {
    NoAuth,
    Internal { ierror: String },
}

pub fn unjoin_club(
    db: &mut DataState,
    auth: &AuthState,
    req: UnjoinClub,
) -> Result<UnjoinClubOut, UnjoinClubError> {
    let Some(user_id) = auth.check_user_with_token(&req.token) else {
        Err(UnjoinClubError::NoAuth)?
    };
    ClubMembers::remove(db, &user_id, &req.club_id).map_err(|e| UnjoinClubError::Internal {
        ierror: e.to_string(),
    })?;
    Ok(UnjoinClubOut {})
}
