use super::*;
use bimap::BiHashMap;
use std::{
    collections::HashMap,
    time::{Duration, SystemTime},
};

/// Stores users that have been created but not verified.
#[derive(Debug, Default)]
pub struct AccountLimboState {
    account_codes: BiHashMap<String, UserId>,
    account_code_times: HashMap<UserId, SystemTime>,
}

/// How long until a verification code is allowed to be swept.
/// This does not necessarily mean that the verification code will expire after this duration.
const ACCOUNT_VERIFICATION_EXPIRE: Duration = Duration::from_secs(900);
/// The length of a verification code.
const ACCOUNT_VERIFICATION_CODE_LENGTH: usize = 6;

impl AccountLimboState {
    pub fn get_user_id(&self, code: &str) -> Option<&UserId> {
        self.account_codes.get_by_left(code)
    }

    pub fn get_code(&self, user_id: &UserId) -> Option<&String> {
        self.account_codes.get_by_right(user_id)
    }

    pub fn get_time(&self, user_id: &UserId) -> Option<&SystemTime> {
        self.account_code_times.get(user_id)
    }

    /// Verify a user without any checks.
    pub fn unchecked_verify_user(
        &mut self,
        db: &mut DataStateInstance,
        code: &str,
        user: &UserId,
    ) -> Result<(), DatabaseError> {
        use crate::schema::users::dsl;

        self.account_codes.remove_by_left(code);
        self.account_code_times.remove(user);

        diesel::update(dsl::users.filter(dsl::id.eq(user.0)))
            .set(dsl::is_verified.eq(true))
            .execute(&mut db.db)
            .map(|_| ())
            .map_err(DatabaseError::from)
    }

    /// Add a user that must be verified.
    /// This method **must** be called after calling [`create_user`].
    /// This also means that account verification codes can be overwritten.
    pub fn push_user(&mut self, user: UserId) -> String {
        let code = generate_token_alphanumeric(ACCOUNT_VERIFICATION_CODE_LENGTH);
        self.account_codes.insert(code.clone(), user);
        self.account_code_times.insert(user, SystemTime::now());
        code
    }

    /// Discard expired accounts verification codes as well as those accounts using the default expiration duration.
    pub fn collect_garbage(&mut self, db: &mut DataStateInstance) {
        self.collect_garbage_with_expire(db, ACCOUNT_VERIFICATION_EXPIRE);
    }

    /// Discard expired accounts verification codes as well as those accounts.
    pub fn collect_garbage_with_expire(&mut self, db: &mut DataStateInstance, expire: Duration) {
        let now = SystemTime::now();
        self.account_codes = self
            .account_codes
            .clone()
            .into_iter()
            .filter(|(_, id)| {
                let time = self.get_time(id).unwrap();
                let res = (time.elapsed().unwrap() - now.elapsed().unwrap()) < expire;
                if !res {
                    User::remove(db, *id).unwrap();
                }
                res
            })
            .collect::<BiHashMap<_, _>>();
    }

    /// Waive all users that are apart of [`AccountLimboState`].
    /// Mainly for the purposes of testing.
    /// The user is not authenticated.
    pub async fn waive_user_verification(&mut self, db: &mut DataStateInstance) {
        for (code, _) in self.account_codes.clone().iter() {
            verify_account(db, None, self, VerifyAccount { code: code.clone() })
                .await
                .unwrap();
        }
    }
}
