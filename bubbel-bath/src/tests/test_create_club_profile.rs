use super::*;

#[test]
#[serial_test::serial]
pub fn test_set_club_profile() {
    let mut db = new_data_state();
    let mut auth = AuthState::default();
    let mut acc_limbo = AccountLimboState::default();

    let acc1_id = create_user(
        &mut db,
        CreateUser {
            email: "g@gmail.com".to_owned(),
            username: "davnotdev".to_owned(),
            password: "passwordnot123".to_owned(),
        },
    )
    .unwrap()
    .user_id;
    acc_limbo.push_user(acc1_id);

    let acc2_id = create_user(
        &mut db,
        CreateUser {
            email: "pg@gmail.com".to_owned(),
            username: "davnotdev2".to_owned(),
            password: "passwordnot123".to_owned(),
        },
    )
    .unwrap()
    .user_id;
    acc_limbo.push_user(acc2_id);

    acc_limbo.waive_user_verification(&mut db);

    let acc1 = auth_user(
        &mut db,
        &mut auth,
        AuthUser {
            username: "davnotdev".to_owned(),
            password: "passwordnot123".to_owned(),
        },
    )
    .unwrap()
    .token;

    let acc2 = auth_user(
        &mut db,
        &mut auth,
        AuthUser {
            username: "davnotdev2".to_owned(),
            password: "passwordnot123".to_owned(),
        },
    )
    .unwrap()
    .token;

    let res = create_club(
        &mut db,
        &auth,
        CreateClub {
            token: acc1.clone(),
            name: "MyClub".to_owned(),
        },
    );
    assert_eq!(res, Ok(CreateClubOut { club_id: ClubId(1) }));
    let club_id = res.unwrap().club_id;

    assert_eq!(
        set_club_profile(
            &mut db,
            &auth,
            SetClubProfile {
                token: acc1.clone(),
                club_id,
                profile: ClubProfilePartial {
                    name: Some("YourClub".to_owned()),
                    ..Default::default()
                }
            }
        ),
        Ok(SetClubProfileOut {})
    );
    assert_eq!(
        set_club_profile(
            &mut db,
            &auth,
            SetClubProfile {
                token: acc2.clone(),
                club_id: ClubId(99),
                profile: ClubProfilePartial {
                    name: Some("YourClub".to_owned()),
                    ..Default::default()
                }
            }
        ),
        Err(SetClubProfileError::ClubNotFound)
    );
    assert_eq!(
        set_club_profile(
            &mut db,
            &auth,
            SetClubProfile {
                token: acc2.clone(),
                club_id,
                profile: ClubProfilePartial {
                    name: Some("YourClub".to_owned()),
                    ..Default::default()
                }
            }
        ),
        Err(SetClubProfileError::NoAuthOwner)
    );

    assert_eq!(
        get_club_profile(
            &mut db,
            &auth,
            GetClubProfile {
                token: None,
                club_id,
            }
        ),
        Ok(GetClubProfileOut {
            profile: ClubProfile {
                owner: acc1_id.0,
                name: "YourClub".to_owned(),
                description: None,
                display_name: None,
                pfp: None,
                banner: None
            }
        })
    );

    assert_eq!(
        delete_club(
            &mut db,
            &auth,
            DeleteClub {
                token: acc1,
                club_id,
            }
        ),
        Ok(())
    );
    assert_eq!(
        get_club_profile(
            &mut db,
            &auth,
            GetClubProfile {
                token: None,
                club_id: ClubId(1),
            }
        ),
        Err(GetClubProfileError::ClubNotFound)
    );
}