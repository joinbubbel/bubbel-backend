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
    pub fn get_code(&self, user: &UserId) -> Option<&String> {
        self.account_codes.get(user).map(|(code, _)| code)
    }

    pub fn unchecked_verify_user(
        &mut self,
        db: &mut DataState,
        user: &UserId,
    ) -> Result<(), DatabaseError> {
        use crate::schema::users::dsl;

        self.account_codes.remove(user);

        diesel::update(dsl::users.filter(dsl::id.eq(user.0)))
            .set(dsl::is_verified.eq(true))
            .execute(&mut db.db)
            .map(|_| ())
            .map_err(DatabaseError::from)
    }

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

    pub fn waive_user_verification(&mut self, db: &mut DataState) {
        self.account_codes
            .clone()
            .iter()
            .for_each(|(user_id, (code, _))| {
                verify_user(
                    db,
                    self,
                    VerifyAccount {
                        code: code.clone(),
                        user_id: *user_id,
                    },
                )
                .unwrap();
            });
    }
}
