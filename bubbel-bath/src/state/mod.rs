use super::*;
use tokio::sync::{Mutex, RwLock};

mod account_verification;
mod auth;
mod channel;

pub use account_verification::AccountLimboState;
pub use auth::{generate_token_alphanumeric, AuthState, UserToken};
pub use channel::ChannelState;

pub struct InnerState {
    pub db: DataState,
    pub auth: RwLock<AuthState>,
    pub acc_limbo: Mutex<AccountLimboState>,
    pub chs: ChannelState,
}

impl InnerState {
    pub fn new(db_url: &str, user_salt: &str) -> Self {
        Self {
            db: DataState::new(db_url, user_salt),
            auth: RwLock::new(AuthState::default()),
            acc_limbo: Mutex::new(AccountLimboState::default()),
            chs: ChannelState::default(),
        }
    }
}
