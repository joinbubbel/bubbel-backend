use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetClubProfileWithName {
    pub name: String,
    pub token: Option<UserToken>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetClubProfileWithNameOut {
    #[serde(flatten)]
    pub profile: ClubProfile,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum GetClubProfileWithNameError {
    ClubNotFound,
    Internal { ierror: String },
}

pub fn get_club_profile_with_name(
    db: &mut DataStateInstance,
    _auth: &AuthState,
    req: GetClubProfileWithName,
) -> Result<GetClubProfileWithNameOut, GetClubProfileWithNameError> {
    let Some(club_id) = ClubProfile::get_club_id_with_name(db, &req.name).map_err(|e| {
        GetClubProfileWithNameError::Internal {
            ierror: e.to_string(),
        }
    })?
    else {
        Err(GetClubProfileWithNameError::ClubNotFound)?
    };

    let profile = ClubProfile::get(db, &club_id)
        .map_err(|e| GetClubProfileWithNameError::Internal {
            ierror: e.to_string(),
        })?
        .ok_or(GetClubProfileWithNameError::ClubNotFound)?;

    Ok(GetClubProfileWithNameOut { profile })
}
