use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetUserMessageGroups {
    token: UserToken,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetUserMessageGroupsOut {
    groups: Vec<MessageGroupId>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum GetUserMessageGroupsError {
    NoAuth,
    Internal { ierror: String },
}

pub async fn get_user_message_groups(
    db: &mut DataStateInstance,
    auth: &AuthState,
    req: GetUserMessageGroups,
) -> Result<GetUserMessageGroupsOut, GetUserMessageGroupsError> {
    let Some(user_id) = auth.check_user_with_token(&req.token) else {
        return Err(GetUserMessageGroupsError::NoAuth)?;
    };

    let groups = MessageGroupMember::get_user_message_groups(db, &user_id).map_err(|e| {
        GetUserMessageGroupsError::Internal {
            ierror: e.to_string(),
        }
    })?;

    Ok(GetUserMessageGroupsOut { groups })
}
