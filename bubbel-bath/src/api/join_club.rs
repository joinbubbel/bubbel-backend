use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct JoinClub {
    pub token: UserToken,
    pub club_id: ClubId,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct JoinClubOut {}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum JoinClubError {
    NoAuth,
    AlreadyJoined,
    Internal { ierror: String },
}

pub async fn join_club(
    db: &mut DataStateInstance,
    auth: &AuthState,
    req: JoinClub,
) -> Result<JoinClubOut, JoinClubError> {
    let Some(user_id) = auth.check_user_with_token(&req.token) else {
        Err(JoinClubError::NoAuth)?
    };

    if ClubMembers::is_user_in_club(db, &user_id, &req.club_id).map_err(|e| {
        JoinClubError::Internal {
            ierror: e.to_string(),
        }
    })? {
        Err(JoinClubError::AlreadyJoined)?
    }

    ClubMembers::insert_new(db, &user_id, &req.club_id).map_err(|e| JoinClubError::Internal {
        ierror: e.to_string(),
    })?;

    Ok(JoinClubOut {})
}
