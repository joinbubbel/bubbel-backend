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
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum GetClubProfileError {
    NoAuth,
    ClubNotFound,
    Internal { ierror: String },
}

pub fn get_club_profile(
    db: &mut DataState,
    _auth: &AuthState,
    req: GetClubProfile,
) -> Result<GetClubProfileOut, GetClubProfileError> {
    let Some(profile) =
        ClubProfile::get(db, &req.club_id).map_err(|e| GetClubProfileError::Internal {
            ierror: e.to_string(),
        })? else {
            Err(GetClubProfileError::ClubNotFound)?
        };
    Ok(GetClubProfileOut { profile })
}
