use bimap::BiHashMap;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct UserId(pub i32);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct UserToken(pub String);

#[derive(Debug)]
pub struct AuthState {
    pub tokens: BiHashMap<UserToken, UserId>,
}
