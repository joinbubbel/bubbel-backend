use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetRandomClubs {
    #[serde(default)]
    _ignore: Option<()>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetRandomClubsOut {
    clubs: Vec<(ClubId, ClubProfile)>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum GetRandomClubsError {
    Internal { ierror: String },
}

pub fn get_random_clubs(
    db: &mut DataStateInstance,
    _: GetRandomClubs,
) -> Result<GetRandomClubsOut, GetRandomClubsError> {
    let clubs = ClubProfile::get_random(db).map_err(|e| GetRandomClubsError::Internal {
        ierror: e.to_string(),
    })?;

    Ok(GetRandomClubsOut { clubs })
}
