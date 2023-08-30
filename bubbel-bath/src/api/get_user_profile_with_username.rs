use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetUserProfileWithUsername {
    pub username: String,
    pub token: Option<UserToken>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetUserProfileWithUsernameOut {
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub pfp: Option<String>,
    pub banner: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum GetUserProfileWithUsernameError {
    NoAuth,
    UserNotFound,
    Internal { ierror: String },
}

pub async fn get_user_profile_with_username(
    db: &mut DataStateInstance,
    auth: &AuthState,
    req: GetUserProfileWithUsername,
) -> Result<GetUserProfileWithUsernameOut, GetUserProfileWithUsernameError> {
    let Some(user_id) = User::get_user_id_with_username(db, &req.username).map_err(|e| {
        GetUserProfileWithUsernameError::Internal {
            ierror: e.to_string(),
        }
    })?
    else {
        Err(GetUserProfileWithUsernameError::UserNotFound)?
    };

    let out = get_user_profile(
        db,
        auth,
        GetUserProfile {
            user_id,
            token: req.token,
        },
    )
    .await
    .map_err(|e| match e {
        GetUserProfileError::NoAuth => GetUserProfileWithUsernameError::NoAuth,
        GetUserProfileError::UserNotFound => GetUserProfileWithUsernameError::UserNotFound,
        GetUserProfileError::Internal { ierror } => {
            GetUserProfileWithUsernameError::Internal { ierror }
        }
    })?;

    Ok(GetUserProfileWithUsernameOut {
        display_name: out.display_name,
        description: out.description,
        name: out.name,
        pfp: out.pfp,
        banner: out.banner,
    })
}
