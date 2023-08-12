use super::*;
use regex::RegexBuilder;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct RegexSearchUsers {
    pub batch_index: usize,
    pub regex: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct RegexSearchUsersOut {
    pub users: Vec<(UserId, String)>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum RegexSearchUsersError {
    RegexLimit,
    Internal { ierror: String },
}

const USER_SEARCH_BATCH_SIZE: usize = 30;

pub fn regex_search_users(
    db: &mut DataStateInstance,
    req: RegexSearchUsers,
) -> Result<RegexSearchUsersOut, RegexSearchUsersError> {
    let batch = UserProfile::get_ordered_batch(db, req.batch_index, USER_SEARCH_BATCH_SIZE)
        .map_err(|e| RegexSearchUsersError::Internal {
            ierror: e.to_string(),
        })?;
    //  TODO: Discussion needed on REDOS attacks.
    let regex = RegexBuilder::new(&req.regex)
        .build()
        .map_err(|_| RegexSearchUsersError::RegexLimit)?;
    let users = batch
        .into_iter()
        .filter(|(_, name)| regex.find(name).is_some())
        .collect::<Vec<_>>();
    Ok(RegexSearchUsersOut { users })
}
