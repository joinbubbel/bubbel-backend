use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct DeleteUser {
    pub token: UserToken,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum DeleteUserError {
    NoAuth,
    Internal { ierror: String },
}

pub async fn delete_user(
    db: &mut DataStateInstance,
    auth: &mut AuthState,
    req: DeleteUser,
) -> Result<(), DeleteUserError> {
    let id = auth
        .check_user_with_token(&req.token)
        .ok_or(DeleteUserError::NoAuth)?;
    User::remove(db, id).map_err(|e| DeleteUserError::Internal {
        ierror: e.to_string(),
    })?;
    auth.deauth_user(&req.token);
    Ok(())
}
