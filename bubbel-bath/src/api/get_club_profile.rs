use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetClubProfile {
    pub token: Option<UserToken>,
    pub club_id: ClubId,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetClubProfileOut {
    #[serde(flatten)]
    pub profile: ClubProfile,
    pub is_member: Option<bool>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum GetClubProfileError {
    NoAuth,
    ClubNotFound,
    Internal { ierror: String },
}

pub async fn get_club_profile(
    db: &mut DataStateInstance,
    auth: &AuthState,
    req: GetClubProfile,
) -> Result<GetClubProfileOut, GetClubProfileError> {
    let Some(profile) =
        ClubProfile::get(db, &req.club_id).map_err(|e| GetClubProfileError::Internal {
            ierror: e.to_string(),
        })?
    else {
        Err(GetClubProfileError::ClubNotFound)?
    };

    let is_member = req
        .token
        .and_then(|token| auth.check_user_with_token(&token))
        .map(|user_id| {
            ClubMembers::is_user_in_club(db, &user_id, &req.club_id).map_err(|e| {
                GetClubProfileError::Internal {
                    ierror: e.to_string(),
                }
            })
        })
        .transpose()?;

    Ok(GetClubProfileOut { profile, is_member })
}
