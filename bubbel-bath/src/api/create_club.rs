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
    Internal { ierror: String },
}

pub fn create_club(
    db: &mut DataState,
    auth: &AuthState,
    req: CreateClub,
) -> Result<CreateClubOut, CreateClubError> {
    let Some(user_id) = auth.check_user_with_token(&req.token) else {
        Err(CreateClubError::NoAuth)?
    };
    let club_id =
        ClubProfile::insert_new(db, &user_id, req.name).map_err(|e| CreateClubError::Internal {
            ierror: e.to_string(),
        })?;
    Ok(CreateClubOut { club_id })
}
