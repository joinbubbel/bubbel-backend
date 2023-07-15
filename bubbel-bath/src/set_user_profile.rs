use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct SetUserProfile {
    pub token: UserToken,
    pub display_name: Option<String>,
    pub name: Option<String>,
    pub pfp: Option<String>,
    pub banner: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum SetUserProfileError {
    NoAuth,
    Internal { ierror: String },
}

pub fn set_user_profile(
    db: &mut DataState,
    auth: &AuthState,
    req: SetUserProfile,
) -> Result<(), SetUserProfileError> {
    let user_id = auth
        .check_user_with_token(&req.token)
        .ok_or(SetUserProfileError::NoAuth)?;

    let partial_user_profile = UserProfile {
        user_id: user_id.0,
        name: req.name,
        display_name: req.display_name,
        pfp: req.pfp,
        banner: req.banner,
    };

    partial_user_profile
        .update_partial(db)
        .map_err(|e| SetUserProfileError::Internal {
            ierror: e.to_string(),
        })?;

    Ok(())
}
