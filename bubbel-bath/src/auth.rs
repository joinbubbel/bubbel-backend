use super::*;
use bimap::BiHashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct UserId(pub i32);

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct UserToken(pub String);

#[derive(Debug, Default)]
pub struct AuthState {
    pub tokens: BiHashMap<UserToken, UserId>,
}
