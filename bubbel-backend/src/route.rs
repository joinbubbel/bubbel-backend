use super::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

type Empty = ();

macro_rules! route {
    ($ROUTER: expr, $CODEGENCTX: expr, $CODEGEN_FN_NAME: expr, $ROUTE: expr, $REQCALL: expr,
     $REQIN: ident,
     $RESOUT: ident,
     $RESERROR: ident,
     $IN: ident,
     $OUT: ident
     ) => {{
        #[derive(Debug, Serialize, Deserialize, JsonSchema)]
        pub struct $IN {
            #[serde(flatten)]
            pub req: $REQIN,
        }

        #[derive(Debug, Serialize, Deserialize, JsonSchema)]
        pub struct $OUT {
            pub error: Option<$RESERROR>,
            pub res: Option<$RESOUT>,
        }

        async fn f(State(state): State<Arc<AppState>>, Json(req): Json<$IN>) -> Json<$OUT> {
            let mut debug = state.debug.write().await;
            debug.push_incoming(&req);
            debug!("API Call incoming: {} req: {:?}", $ROUTE, req);

            #[allow(clippy::redundant_closure_call)]
            let res = match ($REQCALL)(&state, req).await {
                Ok(res) => $OUT {
                    error: None,
                    res: Some(res),
                },
                Err(e) => $OUT {
                    error: Some(e),
                    res: None,
                },
            };
            debug!("API Call outgoing: {} res: {:?}", $ROUTE, res);
            debug.push_outgoing(&res);

            Json(res)
        }
        add_codegen_endpoint!($CODEGENCTX, $CODEGEN_FN_NAME, $ROUTE, $IN, $OUT);
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
        async move |state: &AppState, req: InCreateUser| {
            let mut db = state.db.spawn();
            create_user(&mut db, req.req)
        },
        CreateUser,
        CreateUserOut,
        CreateUserError,
        InCreateUser,
        ResCreateUser
    );
    route!(
        router,
        codegen_ctx,
        "bubbelApiAuthUser",
        "/api/auth_user",
        async move |state: &AppState, req: InAuthUser| {
            let mut db = state.db.spawn();
            let mut auth = state.auth.write().await;
            auth_user(&mut db, &mut auth, req.req)
        },
        AuthUser,
        AuthUserOut,
        AuthUserError,
        InAuthUser,
        ResAuthUser
    );
    route!(
        router,
        codegen_ctx,
        "bubbelApiDeauthUser",
        "/api/deauth_user",
        async move |state: &AppState, req: InDeauthUser| {
            let mut auth = state.auth.write().await;
            deauth_user(&mut auth, req.req);
            Ok(())
        },
        DeauthUser,
        Empty,
        Empty,
        InDeauthUser,
        ResDeauthUser
    );
    route!(
        router,
        codegen_ctx,
        "bubbelApiVerifyAccount",
        "/api/verify_account",
        async move |state: &AppState, req: InVerifyAccount| {
            let mut db = state.db.spawn();
            let mut acc_limbo = state.acc_limbo.lock().await;
            verify_account(&mut db, &mut acc_limbo, req.req)
        },
        VerifyAccount,
        Empty,
        VerifyAccountError,
        InVerifyAccount,
        ResVerifyAccount
    );
    route!(
        router,
        codegen_ctx,
        "bubbelApiSendVerify",
        "/api/send_verify",
        async move |state: &AppState, req: InSendVerify| {
            let mut db = state.db.spawn();
            let mut acc_limbo = state.acc_limbo.lock().await;
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
        SendVerify,
        Empty,
        SendVerifyError,
        InSendVerify,
        ResSendVerify
    );
    route!(
        router,
        codegen_ctx,
        "bubbelApiSetUserProfile",
        "/api/set_user_profile",
        async move |state: &AppState, req: InSetUserProfile| {
            let mut db = state.db.spawn();
            let auth = state.auth.read().await;
            set_user_profile(&mut db, &auth, req.req)
        },
        SetUserProfile,
        Empty,
        SetUserProfileError,
        InSetUserProfile,
        ResSetUserProfile
    );
    route!(
        router,
        codegen_ctx,
        "bubbelApiGetUserProfile",
        "/api/get_user_profile",
        async move |state: &AppState, req: InGetUserProfile| {
            let mut db = state.db.spawn();
            let auth = state.auth.read().await;
            get_user_profile(&mut db, &auth, req.req)
        },
        GetUserProfile,
        GetUserProfileOut,
        GetUserProfileError,
        InGetUserProfile,
        ResGetUserProfile
    );
    route!(
        router,
        codegen_ctx,
        "bubbelApiDeleteUser",
        "/api/delete_user",
        async move |state: &AppState, req: InDeleteUser| {
            let mut db = state.db.spawn();
            let mut auth = state.auth.write().await;
            delete_user(&mut db, &mut auth, req.req)
        },
        DeleteUser,
        Empty,
        DeleteUserError,
        InDeleteUser,
        ResDeleteUser
    );
    route!(
        router,
        codegen_ctx,
        "bubbelApiCreateClub",
        "/api/create_club",
        async move |state: &AppState, req: InCreateClub| {
            let mut db = state.db.spawn();
            let auth = state.auth.read().await;
            create_club(&mut db, &auth, req.req)
        },
        CreateClub,
        CreateClubOut,
        CreateClubError,
        InCreateClub,
        ResCreateClub
    );
    route!(
        router,
        codegen_ctx,
        "bubbelApiGetClubProfile",
        "/api/get_club_profile",
        async move |state: &AppState, req: InGetClubProfile| {
            let mut db = state.db.spawn();
            let auth = state.auth.read().await;
            get_club_profile(&mut db, &auth, req.req)
        },
        GetClubProfile,
        GetClubProfileOut,
        GetClubProfileError,
        InGetClubProfile,
        ResGetClubProfile
    );
    route!(
        router,
        codegen_ctx,
        "bubbelApiSetClubProfile",
        "/api/set_club_profile",
        async move |state: &AppState, req: InSetClubProfile| {
            let mut db = state.db.spawn();
            let auth = state.auth.read().await;
            set_club_profile(&mut db, &auth, req.req)
        },
        SetClubProfile,
        SetClubProfileOut,
        SetClubProfileError,
        InSetClubProfile,
        ResSetClubProfile
    );
    route!(
        router,
        codegen_ctx,
        "bubbelApiDeleteClub",
        "/api/delete_club",
        async move |state: &AppState, req: InDeleteClub| {
            let mut db = state.db.spawn();
            let auth = state.auth.read().await;
            delete_club(&mut db, &auth, req.req)
        },
        DeleteClub,
        Empty,
        DeleteClubError,
        InDeleteClub,
        ResDeleteClub
    );
    route!(
        router,
        codegen_ctx,
        "bubbelApiGetUserProfileWithUsername",
        "/api/get_user_profile_with_username",
        async move |state: &AppState, req: InGetUserProfileWithUsername| {
            let mut db = state.db.spawn();
            let auth = state.auth.read().await;
            get_user_profile_with_username(&mut db, &auth, req.req)
        },
        GetUserProfileWithUsername,
        GetUserProfileWithUsernameOut,
        GetUserProfileWithUsernameError,
        InGetUserProfileWithUsername,
        ResGetUserProfileWithUsername
    );
    route!(
        router,
        codegen_ctx,
        "bubbelApiAddFriendConnection",
        "/api/add_friend_connection",
        async move |state: &AppState, req: InAddFriendConnection| {
            let mut db = state.db.spawn();
            let auth = state.auth.read().await;
            add_friend_connection(&mut db, &auth, req.req)
        },
        AddFriendConnection,
        AddFriendConnectionOut,
        AddFriendConnectionError,
        InAddFriendConnection,
        ResAddFriendConnection
    );
    route!(
        router,
        codegen_ctx,
        "bubbelApiGetFriendConnections",
        "/api/get_friend_connections",
        async move |state: &AppState, req: InGetFriendConnections| {
            let mut db = state.db.spawn();
            let auth = state.auth.read().await;
            get_friend_connections(&mut db, &auth, req.req)
        },
        GetFriendConnections,
        GetFriendConnectionsOut,
        GetFriendConnectionsError,
        InGetFriendConnections,
        ResGetFriendConnections
    );
    route!(
        router,
        codegen_ctx,
        "bubbelApiRemoveFriend",
        "/api/remove_friend",
        async move |state: &AppState, req: InRemoveFriend| {
            let mut db = state.db.spawn();
            let auth = state.auth.read().await;
            remove_friend(&mut db, &auth, req.req)
        },
        RemoveFriend,
        RemoveFriendOut,
        RemoveFriendError,
        InRemoveFriend,
        ResRemoveFriend
    );
    route!(
        router,
        codegen_ctx,
        "bubbelApiJoinClub",
        "/api/join_club",
        async move |state: &AppState, req: InJoinClub| {
            let mut db = state.db.spawn();
            let auth = state.auth.read().await;
            join_club(&mut db, &auth, req.req)
        },
        JoinClub,
        JoinClubOut,
        JoinClubError,
        InJoinClub,
        ResJoinClub
    );
    route!(
        router,
        codegen_ctx,
        "bubbelApiUnjoinClub",
        "/api/unjoin_club",
        async move |state: &AppState, req: InUnjoinClub| {
            let mut db = state.db.spawn();
            let auth = state.auth.read().await;
            unjoin_club(&mut db, &auth, req.req)
        },
        UnjoinClub,
        UnjoinClubOut,
        UnjoinClubError,
        InUnjoinClub,
        ResUnjoinClub
    );
    route!(
        router,
        codegen_ctx,
        "bubbelApiGetClubMembers",
        "/api/get_club_members",
        async move |state: &AppState, req: InGetClubMembers| {
            let mut db = state.db.spawn();
            get_club_members(&mut db, req.req)
        },
        GetClubMembers,
        GetClubMembersOut,
        GetClubMembersError,
        InGetClubMembers,
        ResGetClubMembers
    );
    route!(
        router,
        codegen_ctx,
        "bubbelApiGetUserClubs",
        "/api/get_user_clubs",
        async move |state: &AppState, req: InGetUserClubs| {
            let mut db = state.db.spawn();
            get_user_clubs(&mut db, req.req)
        },
        GetUserClubs,
        GetUserClubsOut,
        GetUserClubsError,
        InGetUserClubs,
        ResGetUserClubs
    );
    route!(
        router,
        codegen_ctx,
        "bubbelApiRegexSearchClubs",
        "/api/regex_search_clubs",
        async move |state: &AppState, req: InRegexSearchClubs| {
            let mut db = state.db.spawn();
            regex_search_clubs(&mut db, req.req)
        },
        RegexSearchClubs,
        RegexSearchClubsOut,
        RegexSearchClubsError,
        InRegexSearchClubs,
        ResRegexSearchClubs
    );
    route!(
        router,
        codegen_ctx,
        "bubbelApiRegexSearchUsers",
        "/api/regex_search_users",
        async move |state: &AppState, req: InRegexSearchUsers| {
            let mut db = state.db.spawn();
            regex_search_users(&mut db, req.req)
        },
        RegexSearchUsers,
        RegexSearchUsersOut,
        RegexSearchUsersError,
        InRegexSearchUsers,
        ResRegexSearchUsers
    );
    route!(
        router,
        codegen_ctx,
        "bubbelApiGetRandomClubs",
        "/api/get_random_clubs",
        async move |state: &AppState, req: InGetRandomClubs| {
            let mut db = state.db.spawn();
            get_random_clubs(&mut db, req.req)
        },
        GetRandomClubs,
        GetRandomClubsOut,
        GetRandomClubsError,
        InGetRandomClubs,
        ResGetRandomClubs
    );
    route!(
        router,
        codegen_ctx,
        "bubbelApiCheckToken",
        "/api/check_token",
        async move |state: &AppState, req: InCheckToken| {
            let auth = state.auth.read().await;
            check_token(&auth, req.req)
        },
        CheckToken,
        CheckTokenOut,
        CheckTokenError,
        InCheckToken,
        ResCheckToken
    );
    route!(
        router,
        codegen_ctx,
        "bubbelApiUnsafeAddFile",
        "/api/unsafe_add_file",
        async move |_, req: InUnsafeAddFile| { unsafe_add_file(req.req) },
        UnsafeAddFile,
        UnsafeAddFileOut,
        UnsafeAddFileError,
        InUnsafeAddFile,
        ResUnsafeAddFile
    );
    router
}
