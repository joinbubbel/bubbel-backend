use super::*;

#[tokio::test]
#[serial_test::serial]
pub async fn test_resend_verify() {
    let dbs = new_data_state();
    let mut db = dbs.spawn();
    let mut acc_limbo = AccountLimboState::default();

    let acc = create_user(
        &mut db,
        CreateUser {
            email: "a@gmail.com".to_owned(),
            username: "usr21p1".to_owned(),
            password: "password123".to_owned(),
        },
    )
    .await
    .unwrap()
    .user_id;
    let old_code = acc_limbo.push_user(acc);

    assert_eq!(
        send_verify(&mut acc_limbo, SendVerify { user_id: acc }).await,
        Err(SendVerifyError::ResendTooSoon)
    );

    assert_eq!(
        send_verify_with_resend_time(
            &mut acc_limbo,
            SendVerify { user_id: acc },
            std::time::Duration::from_secs(0)
        ),
        Ok(())
    );

    assert_eq!(
        verify_account(
            &mut db,
            None,
            &mut acc_limbo,
            VerifyAccount { code: old_code },
        )
        .await,
        Err(VerifyAccountError::CodeTimedOutOrAlreadyVerifiedOrInvalidCode)
    );

    let new_code = acc_limbo.get_code(&acc).unwrap().clone();

    assert!(verify_account(
        &mut db,
        None,
        &mut acc_limbo,
        VerifyAccount { code: new_code },
    )
    .await
    .is_ok(),);
}
