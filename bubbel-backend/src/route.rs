use super::*;

macro_rules! route {
    ($ROUTER: expr, $ROUTE: expr, $REQCALL: expr, $REQ: ident, $RES: ident) => {{
        async fn f(State(state): State<Arc<AppState>>, Json(req): Json<$REQ>) -> Json<$RES> {
            let mut debug = state.debug.write().unwrap();
            debug.push_incoming(&req);

            #[allow(clippy::redundant_closure_call)]
            let res = match ($REQCALL as fn(&AppState, $REQ) -> Result<_, _>)(&state, req) {
                Ok(res) => $RES {
                    error: None,
                    res: Some(res),
                },
                Err(e) => $RES {
                    error: Some(e),
                    res: None,
                },
            };
            debug.push_outgoing(&res);

            Json(res)
        }
        $ROUTER = $ROUTER.route($ROUTE, post(f));
    }};
}

pub fn configure_routes_with_router(mut router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    route!(
        router,
        "/api/get_user_profile",
        |state, req| {
            let mut db = state.db.spawn();
            let auth = state.auth.read().unwrap();
            get_user_profile(&mut db, &auth, req.req)
        },
        InGetUserProfile,
        ResGetUserProfile
    );
    route!(
        router,
        "/api/create_user",
        |state, req| {
            let mut db = state.db.spawn();
            create_user(&mut db, req.req)
        },
        InCreateUser,
        ResCreateUser
    );
    route!(
        router,
        "/api/auth_user",
        |state, req| {
            let mut db = state.db.spawn();
            let mut auth = state.auth.write().unwrap();
            auth_user(&mut db, &mut auth, req.req)
        },
        InAuthUser,
        ResAuthUser
    );
    route!(
        router,
        "/api/deauth_user",
        |state, req| {
            let mut auth = state.auth.write().unwrap();
            deauth_user(&mut auth, req.req);
            Ok(())
        },
        InDeauthUser,
        ResDeauthUser
    );
    route!(
        router,
        "/api/verify_user",
        |state, req| {
            let mut db = state.db.spawn();
            let mut acc_limbo = state.acc_limbo.lock().unwrap();
            verify_account(&mut db, &mut acc_limbo, req.req)
        },
        InVerifyAccount,
        ResVerifyAccount
    );
    route!(
        router,
        "/api/send_verify",
        |state, req| {
            let mut db = state.db.spawn();
            let mut acc_limbo = state.acc_limbo.lock().unwrap();
            let mut run = || {
                let user = User::get(&mut db, req.req.user_id)
                    .map_err(|e| SendVerifyError::Internal {
                        ierror: e.to_string(),
                    })?
                    .ok_or(SendVerifyError::UserNotFound)?;
                send_verify(&mut acc_limbo, req.req.clone())?;

                let code = acc_limbo.get_code(&req.req.user_id).unwrap();

                if email::send_verify_account_email(
                    &state.account_verification_email,
                    &state.account_verification_email_password,
                    &user.email,
                    code,
                )
                .is_err()
                {
                    User::remove(&mut db, req.req.user_id).map_err(|e| SendVerifyError::Internal {
                        ierror: e.to_string(),
                    })
                } else {
                    Ok(())
                }
            };
            run()
        },
        InSendVerify,
        ResSendVerify
    );
    route!(
        router,
        "/api/set_user_profile",
        |state, req| {
            let mut db = state.db.spawn();
            let auth = state.auth.read().unwrap();
            set_user_profile(&mut db, &auth, req.req)
        },
        InSetUserProfile,
        ResSetUserProfile
    );
    route!(
        router,
        "/api/get_user_profile",
        |state, req| {
            let mut db = state.db.spawn();
            let auth = state.auth.read().unwrap();
            get_user_profile(&mut db, &auth, req.req)
        },
        InGetUserProfile,
        ResGetUserProfile
    );
    route!(
        router,
        "/api/delete_user",
        |state, req| {
            let mut db = state.db.spawn();
            let mut auth = state.auth.write().unwrap();
            delete_user(&mut db, &mut auth, req.req)
        },
        InDeleteUser,
        ResDeleteUser
    );
    route!(
        router,
        "/api/create_club",
        |state, req| {
            let mut db = state.db.spawn();
            let auth = state.auth.read().unwrap();
            create_club(&mut db, &auth, req.req)
        },
        InCreateClub,
        ResCreateClub
    );
    route!(
        router,
        "/api/get_club_profile",
        |state, req| {
            let mut db = state.db.spawn();
            let auth = state.auth.read().unwrap();
            get_club_profile(&mut db, &auth, req.req)
        },
        InGetClubProfile,
        ResGetClubProfile
    );
    route!(
        router,
        "/api/set_club_profile",
        |state, req| {
            let mut db = state.db.spawn();
            let auth = state.auth.read().unwrap();
            set_club_profile(&mut db, &auth, req.req)
        },
        InSetClubProfile,
        ResSetClubProfile
    );
    route!(
        router,
        "/api/delete_club",
        |state, req| {
            let mut db = state.db.spawn();
            let auth = state.auth.read().unwrap();
            delete_club(&mut db, &auth, req.req)
        },
        InDeleteClub,
        ResDeleteClub
    );
    route!(
        router,
        "/api/get_user_profile_with_username",
        |state, req| {
            let mut db = state.db.spawn();
            let auth = state.auth.read().unwrap();
            get_user_profile_with_username(&mut db, &auth, req.req)
        },
        InGetUserProfileWithUsername,
        ResGetUserProfileWithUsername
    );
    router
}
