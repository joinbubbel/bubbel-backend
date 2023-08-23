use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetRandomUsers {
    #[serde(default)]
    _ignore: Option<()>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetRandomUsersOut {
    users: Vec<(UserId, UserProfile)>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum GetRandomUsersError {
    Internal { ierror: String },
}

pub fn get_random_users(
    db: &mut DataStateInstance,
    _: GetRandomUsers,
) -> Result<GetRandomUsersOut, GetRandomUsersError> {
    let users = UserProfile::get_random(db).map_err(|e| GetRandomUsersError::Internal {
        ierror: e.to_string(),
    })?;

    Ok(GetRandomUsersOut { users })
}
