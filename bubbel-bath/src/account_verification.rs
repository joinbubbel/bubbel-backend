use super::*;
use std::{
    collections::HashMap,
    time::{Duration, SystemTime},
};

#[derive(Debug, Default)]
pub struct AccountLimboState {
    account_codes: HashMap<UserId, (String, SystemTime)>,
}

const ACCOUNT_VERIFICATION_EXPIRE: Duration = Duration::from_secs(10000);
const ACCOUNT_VERIFICATION_CODE_LENGTH: usize = 8;

impl AccountLimboState {
    pub fn push_user(&mut self, user: UserId) -> String {
        let code = generate_token_alphanumeric(ACCOUNT_VERIFICATION_CODE_LENGTH);
        self.account_codes
            .insert(user, (code.clone(), SystemTime::now()));
        code
    }

    pub fn collect_garbage(&mut self, db: &mut DataState) {
        self.collect_garbage_with_expire(db, ACCOUNT_VERIFICATION_EXPIRE);
    }

    pub fn collect_garbage_with_expire(&mut self, db: &mut DataState, expire: Duration) {
        let now = SystemTime::now();
        self.account_codes.retain(|id, (_, time)| {
            let res = (time.elapsed().unwrap() - now.elapsed().unwrap()) < expire;
            if !res {
                User::remove(db, *id).unwrap();
            }
            res
        });
    }
}

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

pub fn verify_user(
    db: &mut DataState,
    acc_limbo: &mut AccountLimboState,
    req: VerifyAccount,
) -> Result<(), VerifyAccountError> {
    use crate::schema::users::dsl;

    let (code, _) = acc_limbo
        .account_codes
        .get(&req.user_id)
        .ok_or(VerifyAccountError::CodeTimedOutOrInvalidUser)?;
    (code == &req.code)
        .then_some(())
        .ok_or(VerifyAccountError::InvalidCode)?;
    acc_limbo.account_codes.remove(&req.user_id);

    diesel::update(dsl::users.filter(dsl::id.eq(req.user_id.0)))
        .set(dsl::is_verified.eq(true))
        .execute(&mut db.db)
        .map_err(|e| VerifyAccountError::Internal {
            ierror: e.to_string(),
        })?;

    Ok(())
}

pub(crate) fn waive_user_verification(db: &mut DataState, acc_limbo: &mut AccountLimboState) {
    acc_limbo
        .account_codes
        .clone()
        .iter()
        .for_each(|(user_id, (code, _))| {
            verify_user(
                db,
                acc_limbo,
                VerifyAccount {
                    code: code.clone(),
                    user_id: *user_id,
                },
            )
            .unwrap();
        });
}
