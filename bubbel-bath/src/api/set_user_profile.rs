use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct SetUserProfile {
    pub token: UserToken,
    #[serde(flatten)]
    pub profile: UserProfilePartial,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum SetUserProfileError {
    NoAuth,
    Internal { ierror: String },
}

pub fn set_user_profile(
    db: &mut DataStateInstance,
    auth: &AuthState,
    req: SetUserProfile,
) -> Result<(), SetUserProfileError> {
    let user_id = auth
        .check_user_with_token(&req.token)
        .ok_or(SetUserProfileError::NoAuth)?;

    req.profile
        .unchecked_update_partial(db, &user_id)
        .map_err(|e| SetUserProfileError::Internal {
            ierror: e.to_string(),
        })?;

    Ok(())
}
