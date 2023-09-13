use super::*;

//  TODO: This test is for me to quickly debug creating message rooms which is broken atm.

#[tokio::test]
#[serial_test::serial]
pub async fn test_message_rooms() {
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

    let club_id = create_club(
        &mut db,
        &auth,
        CreateClub {
            token: acc1.clone(),
            name: "MyClub".to_owned(),
        },
    )
    .await
    .unwrap()
    .club_id;

    create_message_room(
        &mut db,
        &auth,
        CreateMessageRoom {
            token: acc1.clone(),
            club_id,
            name: "My Cool Channel".to_owned(),
        },
    )
    .await
    .unwrap();

    //  TODO
}
