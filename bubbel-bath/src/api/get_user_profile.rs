use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetUserProfile {
    pub user_id: UserId,
    pub token: Option<UserToken>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetUserProfileOut {
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub pfp: Option<String>,
    pub banner: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum GetUserProfileError {
    NoAuth,
    UserNotFound,
    Internal { ierror: String },
}

pub fn get_user_profile(
    db: &mut DataStateInstance,
    _auth: &AuthState,
    req: GetUserProfile,
) -> Result<GetUserProfileOut, GetUserProfileError> {
    if User::get(db, req.user_id)
        .map_err(|e| GetUserProfileError::Internal {
            ierror: e.to_string(),
        })?
        .is_none()
    {
        Err(GetUserProfileError::UserNotFound)?;
    }
    let profile = UserProfile::get(db, req.user_id)
        .map_err(|e| GetUserProfileError::Internal {
            ierror: e.to_string(),
        })?
        .ok_or(GetUserProfileError::UserNotFound)?;

    Ok(GetUserProfileOut {
        display_name: profile.display_name,
        description: profile.description,
        name: profile.name,
        pfp: profile.pfp,
        banner: profile.banner,
    })
}
