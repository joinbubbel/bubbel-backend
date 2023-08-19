use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct SetClubProfile {
    pub token: UserToken,
    pub club_id: ClubId,
    #[serde(flatten)]
    pub profile: ClubProfilePartial,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct SetClubProfileOut {}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum SetClubProfileError {
    NoAuth,
    /// The user is not the owner and therefore is not authorized.
    NoAuthOwner,
    ClubNotFound,
    Internal {
        ierror: String,
    },

    SettingDCNotSupportedYet,
    SettingOwnerNotSupportedYet,
    SettingNameNotSupportedYet,
}

pub fn set_club_profile(
    db: &mut DataStateInstance,
    auth: &AuthState,
    req: SetClubProfile,
) -> Result<SetClubProfileOut, SetClubProfileError> {
    let Some(profile) =
        ClubProfile::get(db, &req.club_id).map_err(|e| SetClubProfileError::Internal {
            ierror: e.to_string(),
        })?
    else {
        Err(SetClubProfileError::ClubNotFound)?
    };
    let Some(user_id) = auth.check_user_with_token(&req.token) else {
        Err(SetClubProfileError::NoAuth)?
    };
    if user_id != UserId(profile.owner) {
        Err(SetClubProfileError::NoAuthOwner)?
    }

    if let Some(_dc_id) = req.profile.dc_id.as_ref() {
        Err(SetClubProfileError::SettingDCNotSupportedYet)?;
    }
    if let Some(_new_name) = req.profile.name.as_ref() {
        Err(SetClubProfileError::SettingNameNotSupportedYet)?;
    }
    if let Some(_new_owner) = req.profile.owner.as_ref() {
        Err(SetClubProfileError::SettingOwnerNotSupportedYet)?;
        // This code does not function properly.
        // if new_owner != user_id.0 {
        //     match join_club(
        //         db,
        //         auth,
        //         JoinClub {
        //             token: req.token,
        //             club_id: req.club_id,
        //         },
        //     ) {
        //         Ok(_) | Err(JoinClubError::AlreadyJoined) => (),
        //         Err(JoinClubError::NoAuth) => Err(SetClubProfileError::Internal {
        //             ierror: "Reached impossible token case.".to_owned(),
        //         })?,
        //         Err(JoinClubError::Internal { ierror }) => {
        //             Err(SetClubProfileError::Internal { ierror })?
        //         }
        //     };
        // }
    }

    req.profile
        .unchecked_partial_update(db, &req.club_id)
        .map_err(|e| SetClubProfileError::Internal {
            ierror: e.to_string(),
        })?;

    Ok(SetClubProfileOut {})
}
