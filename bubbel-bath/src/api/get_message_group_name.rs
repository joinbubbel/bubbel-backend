use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetMessageGroupName {
    token: UserToken,
    message_group_id: MessageGroupId,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetMessageGroupNameOut {
    display_name: Option<String>,
}

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

    let members = MessageGroupMember::get_message_group_memberships(db, &req.message_group_id)
        .map_err(|e| GetMessageGroupNameError::Internal {
            ierror: e.to_string(),
        })?;

    let display_name = match members.len() {
        1 => {
            let user_id = members[0];
            get_user_profile(
                db,
                auth,
                GetUserProfile {
                    token: None,
                    user_id,
                },
            )
            .await
            .map_err(|e| GetMessageGroupNameError::Internal {
                ierror: format!("Get User Profile failed (1): {:?}", e),
            })?
            .display_name
        }
        2 => {
            let user_id = members
                .iter()
                .copied()
                .find(|&other_id| other_id != user_id)
                .unwrap();
            get_user_profile(
                db,
                auth,
                GetUserProfile {
                    token: None,
                    user_id,
                },
            )
            .await
            .map_err(|e| GetMessageGroupNameError::Internal {
                ierror: format!("Get User Profile failed (2): {:?}", e),
            })?
            .display_name
        }
        _ => {
            MessageGroup::get(db, &req.message_group_id)
                .map_err(|e| GetMessageGroupNameError::Internal {
                    ierror: e.to_string(),
                })?
                .unwrap()
                .name
        }
    };

    Ok(GetMessageGroupNameOut { display_name })
}
