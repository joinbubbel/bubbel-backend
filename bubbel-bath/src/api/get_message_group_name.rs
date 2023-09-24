use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetMessageGroupName {
    token: UserToken,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetMessageGroupNameOut {}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum GetMessageGroupNameError {
    NoAuth,
    Internal { ierror: String },
}

pub async fn get_message_group_name(
    db: &mut DataStateInstance,
    auth: &AuthState,
    req: GetMessageGroupName,
) -> Result<GetMessageGroupNameOut, GetMessageGroupNameError> {
    let Some(user_id) = auth.check_user_with_token(&req.token) else {
        Err(GetMessageGroupNameError::NoAuth)?
    };
    Ok(GetMessageGroupNameOut {})
}
