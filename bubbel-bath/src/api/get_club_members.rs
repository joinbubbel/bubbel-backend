use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetClubMembers {
    club_id: ClubId,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetClubMembersOut {
    users: Vec<UserId>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum GetClubMembersError {
    Internal { ierror: String },
}

pub fn get_club_members(
    db: &mut DataState,
    req: GetClubMembers,
) -> Result<GetClubMembersOut, GetClubMembersError> {
    let users = ClubMembers::get_club_memberships(db, &req.club_id).map_err(|e| {
        GetClubMembersError::Internal {
            ierror: e.to_string(),
        }
    })?;

    Ok(GetClubMembersOut { users })
}
