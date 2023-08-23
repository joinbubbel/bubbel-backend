use super::*;
use bimap::BiHashMap;
use rand::{distributions::Alphanumeric, prelude::*, rngs::OsRng};
use std::{
    collections::HashMap,
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, SystemTime},
};

/// The length of a user token.
const USER_TOKEN_LENGTH: usize = 32;

/// A token representing an authorized, logged in user.
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Hash, PartialEq, Eq)]
pub struct UserToken(pub String);

#[derive(Debug)]
struct TokenData {
    grant_time: SystemTime,
    uses_since_last_collect: AtomicU64,
}

/// Holds the state for logged in users.
#[derive(Debug, Default)]
pub struct AuthState {
    tokens: BiHashMap<UserToken, UserId>,
    token_datas: HashMap<UserToken, TokenData>,
}

/// How long tokens take to expire after a user is "inactive".
/// Inactivity is determined by the number of called to [`AuthState::check_user_with_token`].
const TOKEN_INACTIVE_EXPIRE: Duration = Duration::from_secs(1209600);

impl AuthState {
    /// Check if a user is logged in using `token`.
    pub fn check_user_with_token(&self, token: &UserToken) -> Option<UserId> {
        let res = self.tokens.get_by_left(token).cloned();
        if res.is_some() {
            self.token_datas
                .get(token)
                .unwrap()
                .uses_since_last_collect
                .fetch_add(1, Ordering::SeqCst);
        }
        res
    }

    /// Log a user in without checking anything.
    /// This can overide a user's existing token.
    pub fn unchecked_auth_user(&mut self, user_id: &UserId) -> UserToken {
        let token = UserToken(generate_token_alphanumeric(USER_TOKEN_LENGTH));
        self.tokens.insert(token.clone(), *user_id);
        self.token_datas.insert(
            token.clone(),
            TokenData {
                grant_time: SystemTime::now(),
                uses_since_last_collect: AtomicU64::new(0),
            },
        );
        token
    }

    /// Log a user out.
    pub fn deauth_user(&mut self, token: &UserToken) {
        self.tokens.remove_by_left(token);
        self.token_datas.remove(token);
    }

    /// Discard inactive, expired user tokens codes using the default expiration duration.
    pub fn collect_garbage(&mut self) {
        self.collect_garbage_with_expire(TOKEN_INACTIVE_EXPIRE);
    }

    //  Basically, if you've been using the token for longer than `expire`, you will get logged out
    //  if you haven't used the token since the last round of garbage collection.
    /// Discard inactive, expired user tokens codes.
    pub fn collect_garbage_with_expire(&mut self, expire: Duration) {
        let now = SystemTime::now();
        let removes = self
            .tokens
            .iter()
            .filter_map(|(token, _)| {
                if let Some(data) = self.token_datas.get(token) {
                    ((data.grant_time.elapsed().unwrap() - now.elapsed().unwrap()) > expire
                        && data.uses_since_last_collect.swap(0, Ordering::SeqCst) == 0)
                        .then_some(token)
                } else {
                    Some(token)
                }
            })
            .cloned()
            .collect::<Vec<_>>();
        removes.iter().for_each(|remove| {
            self.deauth_user(remove);
        });
    }
}

/// Securely generate a random alphanumeric string of length `length`.
pub fn generate_token_alphanumeric(length: usize) -> String {
    OsRng
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
