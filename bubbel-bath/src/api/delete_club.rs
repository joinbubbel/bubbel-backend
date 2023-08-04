use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct DeleteClub {
    pub token: UserToken,
    pub club_id: ClubId,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum DeleteClubError {
    NoAuth,
    /// The user is not the owner and therefore is not authorized.
    NoAuthOwner,
    ClubNotFound,
    Internal {
        ierror: String,
    },
}

pub fn delete_club(
    db: &mut DataStateInstance,
    auth: &AuthState,
    req: DeleteClub,
) -> Result<(), DeleteClubError> {
    let Some(profile) =
        ClubProfile::get(db, &req.club_id).map_err(|e| DeleteClubError::Internal {
            ierror: e.to_string(),
        })? else {
            Err(DeleteClubError::ClubNotFound)?
        };
    let Some(user_id) = auth.check_user_with_token(&req.token) else {
        Err(DeleteClubError::NoAuth)?
    };
    if user_id != UserId(profile.owner) {
        Err(DeleteClubError::NoAuthOwner)?
    }

    ClubProfile::remove(db, req.club_id).map_err(|e| DeleteClubError::Internal {
        ierror: e.to_string(),
    })?;

    Ok(())
}
