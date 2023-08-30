use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct CheckToken {
    token: UserToken,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct CheckTokenOut {
    user_id: Option<UserId>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum CheckTokenError {
    Ignore,
}

pub async fn check_token(
    auth: &AuthState,
    req: CheckToken,
) -> Result<CheckTokenOut, CheckTokenError> {
    Ok(CheckTokenOut {
        user_id: auth.check_user_with_token(&req.token),
    })
}
