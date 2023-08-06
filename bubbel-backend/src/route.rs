use super::*;

macro_rules! route {
    ($ROUTER: expr, $CODEGENCTX: expr, $CODEGEN_FN_NAME: expr, $ROUTE: expr, $REQCALL: expr, $REQ: ident, $RES: ident) => {{
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
        add_codegen_endpoint!($CODEGENCTX, $CODEGEN_FN_NAME, $ROUTE, $REQ, $RES);
        $ROUTER = $ROUTER.route($ROUTE, post(f));
    }};
}

pub fn configure_routes_with_router(
    mut router: Router<Arc<AppState>>,
    codegen_ctx: &mut CodegenContext,
) -> Router<Arc<AppState>> {
    route!(
        router,
        codegen_ctx,
        "bubbelApiCreateUser",
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
        codegen_ctx,
        "bubbelApiAuthUser",
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
        codegen_ctx,
        "bubbelApiDeauthUser",
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
        codegen_ctx,
        "bubbelApiVerifyAccount",
        "/api/verify_account",
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
        codegen_ctx,
        "bubbelApiSendVerify",
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
        codegen_ctx,
        "bubbelApiSetUserProfile",
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
        codegen_ctx,
        "bubbelApiGetUserProfile",
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
        codegen_ctx,
        "bubbelApiDeleteUser",
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
        codegen_ctx,
        "bubbelApiCreateClub",
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
        codegen_ctx,
        "bubbelApiGetClubProfile",
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
        codegen_ctx,
        "bubbelApiSetClubProfile",
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
        codegen_ctx,
        "bubbelApiDeleteClub",
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
        codegen_ctx,
        "bubbelApiGetUserProfileWithUsername",
        "/api/get_user_profile_with_username",
        |state, req| {
            let mut db = state.db.spawn();
            let auth = state.auth.read().unwrap();
            get_user_profile_with_username(&mut db, &auth, req.req)
        },
        InGetUserProfileWithUsername,
        ResGetUserProfileWithUsername
    );
    route!(
        router,
        codegen_ctx,
        "bubbelApiAddFriendConnection",
        "/api/add_friend_connection",
        |state, req| {
            let mut db = state.db.spawn();
            let auth = state.auth.read().unwrap();
            add_friend_connection(&mut db, &auth, req.req)
        },
        InAddFriendConnection,
        ResAddFriendConnection
    );
    route!(
        router,
        codegen_ctx,
        "bubbelApiGetFriendConnections",
        "/api/get_friend_connections",
        |state, req| {
            let mut db = state.db.spawn();
            let auth = state.auth.read().unwrap();
            get_friend_connections(&mut db, &auth, req.req)
        },
        InGetFriendConnections,
        ResGetFriendConnections
    );
    route!(
        router,
        codegen_ctx,
        "bubbelApiRemoveFriend",
        "/api/remove_friend",
        |state, req| {
            let mut db = state.db.spawn();
            let auth = state.auth.read().unwrap();
            remove_friend(&mut db, &auth, req.req)
        },
        InRemoveFriend,
        ResRemoveFriend
    );
    route!(
        router,
        codegen_ctx,
        "bubbelApiJoinClub",
        "/api/join_club",
        |state, req| {
            let mut db = state.db.spawn();
            let auth = state.auth.read().unwrap();
            join_club(&mut db, &auth, req.req)
        },
        InJoinClub,
        ResJoinClub
    );
    route!(
        router,
        codegen_ctx,
        "bubbelApiUnjoinClub",
        "/api/unjoin_club",
        |state, req| {
            let mut db = state.db.spawn();
            let auth = state.auth.read().unwrap();
            unjoin_club(&mut db, &auth, req.req)
        },
        InUnjoinClub,
        ResUnjoinClub
    );
    route!(
        router,
        codegen_ctx,
        "bubbelApiGetClubMembers",
        "/api/get_club_members",
        |state, req| {
            let mut db = state.db.spawn();
            get_club_members(&mut db, req.req)
        },
        InGetClubMembers,
        ResGetClubMembers
    );
    route!(
        router,
        codegen_ctx,
        "bubbelApiGetUserClubs",
        "/api/get_user_clubs",
        |state, req| {
            let mut db = state.db.spawn();
            get_user_clubs(&mut db, req.req)
        },
        InGetUserClubs,
        ResGetUserClubs
    );
    router
}
