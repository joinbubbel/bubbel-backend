use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct VerifyAccount {
    pub code: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct VerifyAccountOut {
    pub token: Option<UserToken>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum VerifyAccountError {
    /// My favorite error message.
    CodeTimedOutOrAlreadyVerifiedOrInvalidCode,
    UserNotFound,
    Internal {
        ierror: String,
    },
}

pub async fn verify_account(
    db: &mut DataStateInstance,
    auth: Option<&mut AuthState>,
    acc_limbo: &mut AccountLimboState,
    req: VerifyAccount,
) -> Result<VerifyAccountOut, VerifyAccountError> {
    let user_id = *acc_limbo
        .get_user_id(&req.code)
        .ok_or(VerifyAccountError::CodeTimedOutOrAlreadyVerifiedOrInvalidCode)?;
    acc_limbo
        .unchecked_verify_user(db, &req.code, &user_id)
        .map_err(|e| VerifyAccountError::Internal {
            ierror: e.to_string(),
        })?;

    if let Some(auth) = auth {
        let token = auth_user_from_user_id(db, auth, &user_id)?;
        Ok(VerifyAccountOut { token: Some(token) })
    } else {
        Ok(VerifyAccountOut { token: None })
    }
}

fn auth_user_from_user_id(
    db: &mut DataStateInstance,
    auth: &mut AuthState,
    user_id: &UserId,
) -> Result<UserToken, VerifyAccountError> {
    use crate::schema::users::dsl;

    let _: bool = dsl::users
        .select(dsl::is_verified)
        .filter(dsl::id.eq(user_id.0))
        .first(&mut db.db)
        .map_err(|e| match DatabaseError::from(e) {
            DatabaseError::NotFound => VerifyAccountError::UserNotFound,
            e => VerifyAccountError::Internal {
                ierror: e.to_string(),
            },
        })?;

    let token = auth.unchecked_auth_user(user_id);
    Ok(token)
}
