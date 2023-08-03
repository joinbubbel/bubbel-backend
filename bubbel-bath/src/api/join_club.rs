use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct JoinClub {
    token: UserToken,
    club_id: ClubId,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct JoinClubOut {}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum JoinClubError {
    NoAuth,
    Internal { ierror: String },
}

pub fn join_club(
    db: &mut DataState,
    auth: &AuthState,
    req: JoinClub,
) -> Result<JoinClubOut, JoinClubError> {
    let Some(user_id) = auth.check_user_with_token(&req.token) else {
        Err(JoinClubError::NoAuth)?
    };
    ClubMembers::insert_new(db, &user_id, &req.club_id).map_err(|e| JoinClubError::Internal {
        ierror: e.to_string(),
    })?;
    Ok(JoinClubOut {})
}
