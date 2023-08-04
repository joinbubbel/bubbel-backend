use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetUserClubs {
    user_id: UserId,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetUserClubsOut {
    clubs: Vec<ClubId>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum GetUserClubsError {
    Internal { ierror: String },
}

pub fn get_user_clubs(
    db: &mut DataStateInstance,
    req: GetUserClubs,
) -> Result<GetUserClubsOut, GetUserClubsError> {
    let clubs =
        ClubMembers::get_user_clubs(db, &req.user_id).map_err(|e| GetUserClubsError::Internal {
            ierror: e.to_string(),
        })?;

    Ok(GetUserClubsOut { clubs })
}
