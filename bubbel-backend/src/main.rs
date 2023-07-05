use axum::{
    extract::{Json, State},
    routing::{get, post},
    Router,
};
use bubbel_bath::*;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex, RwLock},
};

const DB_URL_ENV: &str = "BUBBEL_DATABASE_URL";

pub struct AppState {
    db: Mutex<DataState>,
    auth: RwLock<AuthState>,
}

#[tokio::main]
async fn main() {
    let (_, db_url) = std::env::vars().find(|(k, _)| k == DB_URL_ENV).unwrap();
    let state = Arc::new(AppState {
        db: Mutex::new(DataState::new(&db_url).unwrap()),
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

#[derive(Serialize, Deserialize)]
struct InCreateUser {
    #[serde(flatten)]
    req: CreateUser,
}

#[derive(Serialize, Deserialize)]
struct ResCreateUser {
    error: Option<CreateUserError>,
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

#[derive(Serialize, Deserialize)]
struct InAuthUser {
    #[serde(flatten)]
    req: AuthUser,
}

#[derive(Serialize, Deserialize)]
struct ResAuthUser {
    error: Option<AuthUserError>,
    #[serde(flatten)]
    res: Option<AuthUserOut>,
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

#[derive(Serialize, Deserialize)]
struct InDeauthUser {
    #[serde(flatten)]
    req: DeauthUser,
}

#[derive(Serialize, Deserialize)]
struct ResDeauthUser {
    error: Option<()>,
}

async fn api_deauth_user(
    State(state): State<Arc<AppState>>,
    Json(req): Json<InDeauthUser>,
) -> Json<ResDeauthUser> {
    let mut auth = state.auth.write().unwrap();
    deauth_user(&mut auth, req.req);
    Json(ResDeauthUser { error: None })
}
