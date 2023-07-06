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

const USER_SALT_ENV: &str = "BUBBEL_USER_SALT";
const DB_URL_ENV: &str = "BUBBEL_DATABASE_URL";

pub struct AppState {
    db: Mutex<DataState>,
    auth: RwLock<AuthState>,
}

#[tokio::main]
async fn main() {
    let (_, db_url) = std::env::vars().find(|(k, _)| k == DB_URL_ENV).unwrap();
    let (_, user_salt) = std::env::vars().find(|(k, _)| k == USER_SALT_ENV).unwrap();
    let state = Arc::new(AppState {
        db: Mutex::new(DataState::new(&db_url, &user_salt).unwrap()),
        auth: RwLock::new(AuthState::default()),
    });

    let app = Router::new()
        .route("/", get(root))
        .route("/api/create_user", post(api_create_user))
        .route("/api/auth_user", post(api_auth_user))
        .route("/api/deauth_user", post(api_deauth_user))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World"
}

async fn api_create_user(
    State(state): State<Arc<AppState>>,
    Json(req): Json<InCreateUser>,
) -> Json<ResCreateUser> {
    let mut db = state.db.lock().unwrap();
    Json(match create_user(&mut db, req.req) {
        Ok(_) => ResCreateUser { error: None },
        Err(e) => ResCreateUser { error: Some(e) },
    })
}

async fn api_auth_user(
    State(state): State<Arc<AppState>>,
    Json(req): Json<InAuthUser>,
) -> Json<ResAuthUser> {
    let mut db = state.db.lock().unwrap();
    let mut auth = state.auth.write().unwrap();

    Json(match auth_user(&mut db, &mut auth, req.req) {
        Ok(res) => ResAuthUser {
            error: None,
            res: Some(res),
        },
        Err(e) => ResAuthUser {
            error: Some(e),
            res: None,
        },
    })
}

async fn api_deauth_user(
    State(state): State<Arc<AppState>>,
    Json(req): Json<InDeauthUser>,
) -> Json<ResDeauthUser> {
    let mut auth = state.auth.write().unwrap();
    deauth_user(&mut auth, req.req);
    Json(ResDeauthUser { error: None })
}
