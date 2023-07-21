use super::*;
use bimap::BiHashMap;
use std::{
    collections::HashMap,
    time::{Duration, SystemTime},
};

#[derive(Debug, Default)]
pub struct AccountLimboState {
    account_codes: BiHashMap<String, UserId>,
    account_code_times: HashMap<UserId, SystemTime>,
}

const ACCOUNT_VERIFICATION_EXPIRE: Duration = Duration::from_secs(10000);
const ACCOUNT_VERIFICATION_CODE_LENGTH: usize = 12;

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

    pub fn unchecked_verify_user(
        &mut self,
        db: &mut DataState,
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

    pub fn push_user(&mut self, user: UserId) -> String {
        let code = generate_token_alphanumeric(ACCOUNT_VERIFICATION_CODE_LENGTH);
        self.account_codes.insert(code.clone(), user);
        self.account_code_times.insert(user, SystemTime::now());
        code
    }

    pub fn collect_garbage(&mut self, db: &mut DataState) {
        self.collect_garbage_with_expire(db, ACCOUNT_VERIFICATION_EXPIRE);
    }

    pub fn collect_garbage_with_expire(&mut self, db: &mut DataState, expire: Duration) {
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

    pub fn waive_user_verification(&mut self, db: &mut DataState) {
        self.account_codes.clone().iter().for_each(|(code, _)| {
            verify_account(db, self, VerifyAccount { code: code.clone() }).unwrap();
        });
    }
}
