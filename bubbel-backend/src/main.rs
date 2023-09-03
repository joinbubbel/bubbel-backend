#![feature(async_closure)]

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Json, State,
    },
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use bubbel_bath::*;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};
use tracing::{debug, warn};

#[macro_use]
mod codegen;
mod collect_garbage;
mod dc_api;
mod debug;
mod email;
mod route;

use codegen::{schema_for, CodegenContext, Endpoint};
use debug::{api_debug, DebugState};

const USER_SALT_ENV: &str = "BUBBEL_USER_SALT";
const DB_URL_ENV: &str = "BUBBEL_DATABASE_URL";
const DEBUG_ENABLED_ENV: &str = "BUBBEL_ENABLE_DEBUG_INSPECTOR";
const DEBUG_PASSWORD_ENV: &str = "BUBBEL_DEBUG_INSPECTOR_PASSWORD";
const ACCOUNT_VERIFICATION_FROM_EMAIL: &str = "BUBBEL_ACCOUNT_VERIFICATION_FROM_EMAIL";
const ACCOUNT_VERIFICATION_FROM_EMAIL_PASSWORD: &str =
    "BUBBEL_ACCOUNT_VERIFICATION_FROM_EMAIL_PASSWORD";
const TLS_CERTIFICATE_PATH_ENV: &str = "BUBBEL_TLS_CERT_PATH";
const TLS_KEY_PATH_ENV: &str = "BUBBEL_TLS_KEY_PATH";
const RUST_DOCS_PATH_ENV: &str = "BUBBEL_DOCS_PATH";

pub struct AppState {
    inner: InnerState,

    debug: RwLock<DebugState>,
    account_verification_email: String,
    account_verification_email_password: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    if let Some(codegen_root) = option_env!("BUBBEL_CODEGEN") {
        let mut codegen_ctx = CodegenContext::new();
        let app = Router::new();
        let app = route::configure_routes_with_router(app, &mut codegen_ctx);
        let _ = dc_api::configure_routes_with_router(app, &mut codegen_ctx);
        codegen_ctx.gen_and_write(codegen_root);
        return;
    }

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
        inner: InnerState::new(&db_url, &user_salt),
        debug: RwLock::new(DebugState::new(debug_enabled, debug_password)),
        account_verification_email,
        account_verification_email_password,
    });
    let garbage_state = Arc::clone(&state);

    let cors = CorsLayer::very_permissive();
    let trace = TraceLayer::new_for_http();

    let tls_config = RustlsConfig::from_pem_file(tls_cert_path, tls_key_path)
        .await
        .ok();

    let app = Router::new();
    let mut codegen_ctx = CodegenContext::new();
    let app = route::configure_routes_with_router(app, &mut codegen_ctx);
    let app = dc_api::configure_routes_with_router(app, &mut codegen_ctx);
    let mut app = app
        .route("/", get(root))
        .route("/api/debug", post(api_debug))
        .layer(cors)
        .layer(trace)
        .with_state(Arc::clone(&state));

    let mut tls_app = app.clone();

    app = app.nest_service("/dumpster", ServeDir::new("/bubbel/dumpster"));
    tls_app = tls_app.nest_service("/dumpster", ServeDir::new("/bubbel/dumpster"));

    if let Some(rust_docs_path) = rust_docs_path {
        app = app.nest_service("/docs", ServeDir::new(rust_docs_path.clone()));
        tls_app = tls_app.nest_service("/docs", ServeDir::new(rust_docs_path));
    }

    app = app.nest_service("/unsafe_data", ServeDir::new("/bubbel/unsafe_data"));
    tls_app = tls_app.nest_service("/unsafe_data", ServeDir::new("/bubbel/unsafe_data"));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let tls_addr = SocketAddr::from(([0, 0, 0, 0], 8443));

    tokio::join!(
        axum_server::bind(addr).serve(app.into_make_service()),
        tokio::spawn(async move {
            if let Some(tls_config) = tls_config {
                axum_server::bind_rustls(tls_addr, tls_config)
                    .serve(tls_app.into_make_service())
                    .await
            } else {
                warn!("Running without TLS");
                Ok(()) as Result<(), std::io::Error>
            }
        }),
        collect_garbage::collect_garbage(garbage_state)
    )
    .0
    .unwrap();
}

async fn root() -> &'static str {
    "Hello, World"
}
