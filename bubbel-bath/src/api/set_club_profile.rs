use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct SetClubProfile {
    pub token: UserToken,
    pub club_id: ClubId,
    #[serde(flatten)]
    pub profile: ClubProfilePartial,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct SetClubProfileOut {}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum SetClubProfileError {
    NoAuth,
    /// The user is not the owner and therefore is not authorized.
    NoAuthOwner,
    ClubNotFound,
    Internal {
        ierror: String,
    },
}

pub fn set_club_profile(
    db: &mut DataState,
    auth: &AuthState,
    req: SetClubProfile,
) -> Result<SetClubProfileOut, SetClubProfileError> {
    let Some(profile) =
        ClubProfile::get(db, &req.club_id).map_err(|e| SetClubProfileError::Internal {
            ierror: e.to_string(),
        })? else {
            Err(SetClubProfileError::ClubNotFound)?
        };
    let Some(user_id) = auth.check_user_with_token(&req.token) else {
        Err(SetClubProfileError::NoAuth)?
    };
    if user_id != UserId(profile.owner) {
        Err(SetClubProfileError::NoAuthOwner)?
    }

    req.profile
        .unchecked_partial_update(db, &req.club_id)
        .map_err(|e| SetClubProfileError::Internal {
            ierror: e.to_string(),
        })?;

    Ok(SetClubProfileOut {})
}
