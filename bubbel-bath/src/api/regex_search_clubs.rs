use super::*;
use regex::RegexBuilder;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct RegexSearchClubs {
    pub batch_index: usize,
    pub regex: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct RegexSearchClubsOut {
    pub clubs: Vec<(ClubId, String)>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum RegexSearchClubsError {
    RegexLimit,
    Internal { ierror: String },
}

const CLUB_SEARCH_BATCH_SIZE: usize = 30;

pub fn regex_search_clubs(
    db: &mut DataStateInstance,
    req: RegexSearchClubs,
) -> Result<RegexSearchClubsOut, RegexSearchClubsError> {
    let batch = ClubProfile::get_ordered_batch(db, req.batch_index, CLUB_SEARCH_BATCH_SIZE)
        .map_err(|e| RegexSearchClubsError::Internal {
            ierror: e.to_string(),
        })?;
    //  TODO: Discussion needed on REDOS attacks.
    let regex = RegexBuilder::new(&req.regex)
        .build()
        .map_err(|_| RegexSearchClubsError::RegexLimit)?;
    let clubs = batch
        .into_iter()
        .filter(|(_, name)| regex.find(name).is_some())
        .collect::<Vec<_>>();
    Ok(RegexSearchClubsOut { clubs })
}
