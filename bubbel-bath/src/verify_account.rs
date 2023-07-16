use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct VerifyAccount {
    pub code: String,
    pub user_id: UserId,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum VerifyAccountError {
    InvalidCode,
    CodeTimedOutOrInvalidUser,
    Internal { ierror: String },
}

pub fn verify_account(
    db: &mut DataState,
    acc_limbo: &mut AccountLimboState,
    req: VerifyAccount,
) -> Result<(), VerifyAccountError> {
    let (code, _) = acc_limbo
        .get_code_and_time(&req.user_id)
        .ok_or(VerifyAccountError::CodeTimedOutOrInvalidUser)?;
    (code == &req.code)
        .then_some(())
        .ok_or(VerifyAccountError::InvalidCode)?;
    acc_limbo
        .unchecked_verify_user(db, &req.user_id)
        .map_err(|e| VerifyAccountError::Internal {
            ierror: e.to_string(),
        })?;

    Ok(())
}
