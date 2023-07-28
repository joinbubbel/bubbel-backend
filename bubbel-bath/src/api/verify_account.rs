use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct VerifyAccount {
    pub code: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum VerifyAccountError {
    /// My favorite error message.
    CodeTimedOutOrAlreadyVerifiedOrInvalidCode,
    Internal { ierror: String },
}

pub fn verify_account(
    db: &mut DataState,
    acc_limbo: &mut AccountLimboState,
    req: VerifyAccount,
) -> Result<(), VerifyAccountError> {
    let user_id = *acc_limbo
        .get_user_id(&req.code)
        .ok_or(VerifyAccountError::CodeTimedOutOrAlreadyVerifiedOrInvalidCode)?;
    acc_limbo
        .unchecked_verify_user(db, &req.code, &user_id)
        .map_err(|e| VerifyAccountError::Internal {
            ierror: e.to_string(),
        })?;

    Ok(())
}
