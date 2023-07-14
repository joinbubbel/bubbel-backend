use super::*;
use bimap::BiHashMap;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct UserId(pub i32);

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Hash, PartialEq, Eq)]
pub struct UserToken(pub String);

#[derive(Debug, Default)]
pub struct AuthState {
    pub tokens: BiHashMap<UserToken, UserId>,
}

impl AuthState {
    pub fn collect_garbage(&mut self) {
        todo!()
    }
}
