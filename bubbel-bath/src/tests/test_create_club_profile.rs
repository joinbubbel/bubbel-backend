use super::*;

#[tokio::test]
#[serial_test::serial]
pub async fn test_set_club_profile() {
    let dbs = new_data_state();
    let mut db = dbs.spawn();
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
    .await
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
    .await
    .unwrap()
    .user_id;
    acc_limbo.push_user(acc2_id);

    acc_limbo.waive_user_verification(&mut db).await;

    let acc1 = auth_user(
        &mut db,
        &mut auth,
        AuthUser {
            username: "davnotdev".to_owned(),
            password: "passwordnot123".to_owned(),
        },
    )
    .await
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
    .await
    .unwrap()
    .token;

    let res = create_club(
        &mut db,
        &auth,
        CreateClub {
            token: acc1.clone(),
            name: "MyClub".to_owned(),
        },
    )
    .await;
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
                    display_name: Some("YourClub".to_owned()),
                    ..Default::default()
                }
            }
        )
        .await,
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
                    display_name: Some("YourClub".to_owned()),
                    ..Default::default()
                }
            }
        )
        .await,
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
                    display_name: Some("YourClub".to_owned()),
                    ..Default::default()
                }
            }
        )
        .await,
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
        )
        .await,
        Ok(GetClubProfileOut {
            profile: ClubProfile {
                owner: acc1_id.0,
                dc_id: 1,
                name: "MyClub".to_owned(),
                description: None,
                display_name: Some("YourClub".to_owned()),
                pfp: None,
                banner: None,
            },
            is_member: None,
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
        )
        .await,
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
        )
        .await,
        Err(GetClubProfileError::ClubNotFound)
    )
}
