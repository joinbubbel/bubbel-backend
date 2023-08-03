use axum::{
    extract::{Json, State},
    routing::{get, post},
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use bubbel_backend::*;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex, RwLock},
};
use tower_http::{cors::CorsLayer, services::ServeFile};

mod collect_garbage;
mod debug;
mod email;

use debug::{api_debug, DebugState};

const USER_SALT_ENV: &str = "BUBBEL_USER_SALT";
const DB_URL_ENV: &str = "BUBBEL_DATABASE_URL";
const DEBUG_ENABLED_ENV: &str = "BUBBEL_ENABLE_DEBUG_INSPECTOR";
const DEBUG_PASSWORD_ENV: &str = "BUBBEL_DEBUG_INSPECTOR_PASSWORD";
const ACCOUNT_VERIFICATION_FROM_EMAIL: &str = "BUBBEL_ACCOUNT_VERIFICATION_FROM_EMAIL";
const ACCOUNT_VERIFICATION_FROM_EMAIL_PASSWORD: &str =
    "BUBBEL_ACCOUNT_VERIFICATION_FROM_EMAIL_PASSWORD";
const WAIVE_ALL_ACCOUNT_VERIFICATION: &str = "BUBBEL_ENABLE_WAIVE_ALL_ACCOUNT_VERIFICATION";
const TLS_CERTIFICATE_PATH_ENV: &str = "BUBBEL_TLS_CERT_PATH";
const TLS_KEY_PATH_ENV: &str = "BUBBEL_TLS_KEY_PATH";
const RUST_DOCS_PATH_ENV: &str = "BUBBEL_DOCS_PATH";

pub struct AppState {
    db: Mutex<DataState>,
    auth: RwLock<AuthState>,
    debug: RwLock<DebugState>,
    acc_limbo: Mutex<AccountLimboState>,

    account_verification_email: String,
    account_verification_email_password: String,
    enabled_waive_all_account_verification: bool,
}

#[tokio::main]
async fn main() {
    let (_, db_url) = std::env::vars().find(|(k, _)| k == DB_URL_ENV).unwrap();
    let (_, user_salt) = std::env::vars().find(|(k, _)| k == USER_SALT_ENV).unwrap();
    let debug_enabled = std::env::vars().any(|(k, _)| k == DEBUG_ENABLED_ENV);
    let debug_password = std::env::vars()
        .find(|(k, _)| k == DEBUG_PASSWORD_ENV)
        .map(|(_, p)| p);
    let (_, account_verification_email) = std::env::vars()
        .find(|(k, _)| k == ACCOUNT_VERIFICATION_FROM_EMAIL)
        .unwrap();
    let (_, account_verification_email_password) = std::env::vars()
        .find(|(k, _)| k == ACCOUNT_VERIFICATION_FROM_EMAIL_PASSWORD)
        .unwrap();

    let enabled_waive_all_account_verification =
        std::env::vars().any(|(k, _)| k == WAIVE_ALL_ACCOUNT_VERIFICATION);

    let (_, tls_cert_path) = std::env::vars()
        .find(|(k, _)| k == TLS_CERTIFICATE_PATH_ENV)
        .unwrap();
    let (_, tls_key_path) = std::env::vars()
        .find(|(k, _)| k == TLS_KEY_PATH_ENV)
        .unwrap();

    let rust_docs_path = std::env::vars()
        .find(|(k, _)| k == RUST_DOCS_PATH_ENV)
        .map(|(_, p)| p);

    let state = Arc::new(AppState {
        db: Mutex::new(DataState::new(&db_url, &user_salt).unwrap()),
        auth: RwLock::new(AuthState::default()),
        debug: RwLock::new(DebugState::new(debug_enabled, debug_password)),
        acc_limbo: Mutex::new(AccountLimboState::default()),
        account_verification_email,
        account_verification_email_password,
        enabled_waive_all_account_verification,
    });
    let garbage_state = Arc::clone(&state);

    let cors = CorsLayer::very_permissive();

    let tls_config = RustlsConfig::from_pem_file(tls_cert_path, tls_key_path)
        .await
        .unwrap();

    let mut app = Router::new()
        .route("/", get(root))
        .route(
            "/api/waive_all_account_verification",
            get(get_waive_all_account_verification),
        )
        .route("/api/debug", post(api_debug))
        .route("/api/create_user", post(api_create_user))
        .route("/api/auth_user", post(api_auth_user))
        .route("/api/deauth_user", post(api_deauth_user))
        .route("/api/verify_account", post(api_verify_user))
        .route("/api/send_verify", post(api_send_verify))
        .route("/api/set_user_profile", post(api_set_user_profile))
        .route("/api/get_user_profile", post(api_get_user_profile))
        .route("/api/delete_user", post(api_delete_user))
        .route("/api/create_club", post(api_create_club))
        .route("/api/get_club_profile", post(api_get_club_profile))
        .route("/api/set_club_profile", post(api_set_club_profile))
        .route("/api/delete_club", post(api_delete_club))
        .layer(cors)
        .with_state(state);

    if let Some(rust_docs_path) = rust_docs_path {
        app = app.nest_service(
            "/docs",
            ServeFile::new_with_mime(rust_docs_path, &"text/html".parse::<mime::Mime>().unwrap()),
        );
    }

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    tokio::join!(
        axum_server::bind_rustls(addr, tls_config).serve(app.into_make_service()),
        collect_garbage::collect_garbage(garbage_state)
    )
    .0
    .unwrap();
}

async fn root() -> &'static str {
    "Hello, World"
}

async fn get_waive_all_account_verification(State(state): State<Arc<AppState>>) {
    if state.enabled_waive_all_account_verification {
        let mut db = state.db.lock().unwrap();
        let mut acc_limbo = state.acc_limbo.lock().unwrap();
        acc_limbo.waive_user_verification(&mut db);
    }
}

async fn api_create_user(
    State(state): State<Arc<AppState>>,
    Json(req): Json<InCreateUser>,
) -> Json<ResCreateUser> {
    let mut debug = state.debug.write().unwrap();
    debug.push_incoming(&req);

    let mut db = state.db.lock().unwrap();
    let res = match create_user(&mut db, req.req) {
        Ok(res) => ResCreateUser {
            error: None,
            res: Some(res),
        },
        Err(e) => ResCreateUser {
            error: Some(e),
            res: None,
        },
    };
    debug.push_outgoing(&res);

    Json(res)
}

async fn api_auth_user(
    State(state): State<Arc<AppState>>,
    Json(req): Json<InAuthUser>,
) -> Json<ResAuthUser> {
    let mut debug = state.debug.write().unwrap();
    debug.push_incoming(&req);

    let mut db = state.db.lock().unwrap();
    let mut auth = state.auth.write().unwrap();

    let res = match auth_user(&mut db, &mut auth, req.req) {
        Ok(res) => ResAuthUser {
            error: None,
            res: Some(res),
        },
        Err(e) => ResAuthUser {
            error: Some(e),
            res: None,
        },
    };
    debug.push_outgoing(&res);

    Json(res)
}

async fn api_deauth_user(
    State(state): State<Arc<AppState>>,
    Json(req): Json<InDeauthUser>,
) -> Json<ResDeauthUser> {
    let mut debug = state.debug.write().unwrap();
    debug.push_incoming(&req);

    let mut auth = state.auth.write().unwrap();
    deauth_user(&mut auth, req.req);

    let res = ResDeauthUser {
        error: None,
        res: Some(()),
    };
    debug.push_outgoing(&res);

    Json(res)
}

async fn api_verify_user(
    State(state): State<Arc<AppState>>,
    Json(req): Json<InVerifyAccount>,
) -> Json<ResVerifyAccount> {
    let mut debug = state.debug.write().unwrap();
    debug.push_incoming(&req);

    let mut db = state.db.lock().unwrap();
    let mut acc_limbo = state.acc_limbo.lock().unwrap();

    let res = match verify_account(&mut db, &mut acc_limbo, req.req) {
        Ok(_) => ResVerifyAccount {
            error: None,
            res: Some(()),
        },
        Err(e) => ResVerifyAccount {
            error: Some(e),
            res: Some(()),
        },
    };
    debug.push_outgoing(&res);

    Json(res)
}

async fn api_send_verify(
    State(state): State<Arc<AppState>>,
    Json(req): Json<InSendVerify>,
) -> Json<ResSendVerify> {
    let mut debug = state.debug.write().unwrap();
    debug.push_incoming(&req);

    let mut db = state.db.lock().unwrap();
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

    let res = match run() {
        Ok(_) => ResSendVerify {
            error: None,
            res: Some(()),
        },
        Err(e) => ResSendVerify {
            error: Some(e),
            res: Some(()),
        },
    };

    debug.push_outgoing(&res);

    Json(res)
}

async fn api_set_user_profile(
    State(state): State<Arc<AppState>>,
    Json(req): Json<InSetUserProfile>,
) -> Json<ResSetUserProfile> {
    let mut debug = state.debug.write().unwrap();
    debug.push_incoming(&req);

    let mut db = state.db.lock().unwrap();
    let auth = state.auth.read().unwrap();

    let res = match set_user_profile(&mut db, &auth, req.req) {
        Ok(_) => ResSetUserProfile {
            error: None,
            res: Some(()),
        },
        Err(e) => ResSetUserProfile {
            error: Some(e),
            res: Some(()),
        },
    };
    debug.push_outgoing(&res);

    Json(res)
}

async fn api_get_user_profile(
    State(state): State<Arc<AppState>>,
    Json(req): Json<InGetUserProfile>,
) -> Json<ResGetUserProfile> {
    let mut debug = state.debug.write().unwrap();
    debug.push_incoming(&req);

    let mut db = state.db.lock().unwrap();
    let auth = state.auth.read().unwrap();

    let res = match get_user_profile(&mut db, &auth, req.req) {
        Ok(res) => ResGetUserProfile {
            error: None,
            res: Some(res),
        },
        Err(e) => ResGetUserProfile {
            error: Some(e),
            res: None,
        },
    };
    debug.push_outgoing(&res);

    Json(res)
}

async fn api_delete_user(
    State(state): State<Arc<AppState>>,
    Json(req): Json<InDeleteUser>,
) -> Json<ResDeleteUser> {
    let mut debug = state.debug.write().unwrap();
    debug.push_incoming(&req);

    let mut db = state.db.lock().unwrap();
    let mut auth = state.auth.write().unwrap();

    let res = match delete_user(&mut db, &mut auth, req.req) {
        Ok(_) => ResDeleteUser {
            error: None,
            res: Some(()),
        },
        Err(e) => ResDeleteUser {
            error: Some(e),
            res: Some(()),
        },
    };
    debug.push_outgoing(&res);

    Json(res)
}

async fn api_create_club(
    State(state): State<Arc<AppState>>,
    Json(req): Json<InCreateClub>,
) -> Json<ResCreateClub> {
    let mut debug = state.debug.write().unwrap();
    debug.push_incoming(&req);

    let mut db = state.db.lock().unwrap();
    let auth = state.auth.read().unwrap();

    let res = match create_club(&mut db, &auth, req.req) {
        Ok(res) => ResCreateClub {
            error: None,
            res: Some(res),
        },
        Err(e) => ResCreateClub {
            error: Some(e),
            res: None,
        },
    };
    debug.push_outgoing(&res);

    Json(res)
}

async fn api_get_club_profile(
    State(state): State<Arc<AppState>>,
    Json(req): Json<InGetClubProfile>,
) -> Json<ResGetClubProfile> {
    let mut debug = state.debug.write().unwrap();
    debug.push_incoming(&req);

    let mut db = state.db.lock().unwrap();
    let auth = state.auth.read().unwrap();

    let res = match get_club_profile(&mut db, &auth, req.req) {
        Ok(res) => ResGetClubProfile {
            error: None,
            res: Some(res),
        },
        Err(e) => ResGetClubProfile {
            error: Some(e),
            res: None,
        },
    };
    debug.push_outgoing(&res);

    Json(res)
}

async fn api_set_club_profile(
    State(state): State<Arc<AppState>>,
    Json(req): Json<InSetClubProfile>,
) -> Json<ResSetClubProfile> {
    let mut debug = state.debug.write().unwrap();
    debug.push_incoming(&req);

    let mut db = state.db.lock().unwrap();
    let auth = state.auth.read().unwrap();

    let res = match set_club_profile(&mut db, &auth, req.req) {
        Ok(res) => ResSetClubProfile {
            error: None,
            res: Some(res),
        },
        Err(e) => ResSetClubProfile {
            error: Some(e),
            res: None,
        },
    };
    debug.push_outgoing(&res);

    Json(res)
}

async fn api_delete_club(
    State(state): State<Arc<AppState>>,
    Json(req): Json<InDeleteClub>,
) -> Json<ResDeleteClub> {
    let mut debug = state.debug.write().unwrap();
    debug.push_incoming(&req);

    let mut db = state.db.lock().unwrap();
    let auth = state.auth.read().unwrap();

    let res = match delete_club(&mut db, &auth, req.req) {
        Ok(_) => ResDeleteClub {
            error: None,
            res: Some(()),
        },
        Err(e) => ResDeleteClub {
            error: Some(e),
            res: Some(()),
        },
    };
    debug.push_outgoing(&res);

    Json(res)
}
