use super::*;

#[test]
#[serial_test::serial]
pub fn test_resend_verify() {
    let mut db = new_data_state();
    let mut acc_limbo = AccountLimboState::default();

    let acc = create_user(
        &mut db,
        CreateUser {
            email: "a@gmail.com".to_owned(),
            username: "usr21p1".to_owned(),
            password: "password123".to_owned(),
        },
    )
    .unwrap();
    let old_code = acc_limbo.push_user(acc);

    assert_eq!(
        send_verify(&mut acc_limbo, SendVerify { user_id: acc }),
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
            &mut acc_limbo,
            VerifyAccount {
                code: old_code,
                user_id: acc,
            },
        ),
        Err(VerifyAccountError::InvalidCode)
    );

    let (new_code, _) = acc_limbo.get_code_and_time(&acc).unwrap().clone();

    assert_eq!(
        verify_account(
            &mut db,
            &mut acc_limbo,
            VerifyAccount {
                code: new_code,
                user_id: acc,
            },
        ),
        Ok(())
    );
}
