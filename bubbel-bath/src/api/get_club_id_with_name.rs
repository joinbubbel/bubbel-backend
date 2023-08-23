use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetClubIdWithName {
    pub name: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetClubIdWithNameOut {
    pub club_id: ClubId,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum GetClubIdWithNameError {
    ClubNotFound,
    Internal { ierror: String },
}

pub fn get_club_id_with_name(
    db: &mut DataStateInstance,
    req: GetClubIdWithName,
) -> Result<GetClubIdWithNameOut, GetClubIdWithNameError> {
    let Some(club_id) = ClubProfile::get_club_id_with_name(db, &req.name).map_err(|e| {
        GetClubIdWithNameError::Internal {
            ierror: e.to_string(),
        }
    })?
    else {
        Err(GetClubIdWithNameError::ClubNotFound)?
    };

    Ok(GetClubIdWithNameOut { club_id })
}
