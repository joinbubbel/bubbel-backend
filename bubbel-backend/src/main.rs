use axum::{
    extract::{Json, State},
    routing::{get, post},
    Router,
};
use bubbel_backend::*;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex, RwLock},
};
use tower_http::cors::CorsLayer;

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

pub struct AppState {
    db: Mutex<DataState>,
    auth: RwLock<AuthState>,
    debug: RwLock<DebugState>,
    acc_limbo: Mutex<AccountLimboState>,

    account_verification_email: String,
    account_verification_email_password: String,
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

    let state = Arc::new(AppState {
        db: Mutex::new(DataState::new(&db_url, &user_salt).unwrap()),
        auth: RwLock::new(AuthState::default()),
        debug: RwLock::new(DebugState::new(debug_enabled, debug_password)),
        acc_limbo: Mutex::new(AccountLimboState::default()),
        account_verification_email,
        account_verification_email_password,
    });
    let garbage_state = Arc::clone(&state);

    let cors = CorsLayer::very_permissive();

    let app = Router::new()
        .route("/", get(root))
        .route("/api/debug", post(api_debug))
        .route("/api/create_user", post(api_create_user))
        .route("/api/auth_user", post(api_auth_user))
        .route("/api/deauth_user", post(api_deauth_user))
        .route("/api/verify_user", post(api_verify_user))
        .route("/api/set_user_profile", post(api_set_user_profile))
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    tokio::join!(
        axum::Server::bind(&addr).serve(app.into_make_service()),
        collect_garbage::collect_garbage(garbage_state)
    )
    .0
    .unwrap();
}

async fn root() -> &'static str {
    "Hello, World"
}

async fn api_create_user(
    State(state): State<Arc<AppState>>,
    Json(req): Json<InCreateUser>,
) -> Json<ResCreateUser> {
    let mut debug = state.debug.write().unwrap();
    debug.push_incoming(&req);

    let mut db = state.db.lock().unwrap();
    let mut acc_limbo = state.acc_limbo.lock().unwrap();
    let email = req.req.email.clone();
    let res = match create_user(&mut db, req.req) {
        Ok(user_id) => {
            let code = acc_limbo.push_user(user_id);
            if email::send_verify_account_email(
                &state.account_verification_email,
                &state.account_verification_email_password,
                &email,
                &code,
                &user_id.0.to_string(),
            )
            .is_err()
            {
                let rem = User::remove(&mut db, user_id).map_err(|e| CreateUserError::Internal {
                    ierror: e.to_string(),
                });
                if let Err(e) = rem {
                    ResCreateUser { error: Some(e) }
                } else {
                    ResCreateUser {
                        error: Some(CreateUserError::SendVerification),
                    }
                }
            } else {
                ResCreateUser { error: None }
            }
        }
        Err(e) => ResCreateUser { error: Some(e) },
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

    let res = ResDeauthUser { error: None };
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

    let res = match verify_user(&mut db, &mut acc_limbo, req.req) {
        Ok(_) => ResVerifyAccount { error: None },
        Err(e) => ResVerifyAccount { error: Some(e) },
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
        Ok(_) => ResSetUserProfile { error: None },
        Err(e) => ResSetUserProfile { error: Some(e) },
    };
    debug.push_outgoing(&res);

    Json(res)
}
