use super::*;
use std::time::Duration;

pub async fn collect_garbage(app: Arc<AppState>) {
    tokio::join!(collect_auth_garbage(&app), collect_acc_limbo_garbage(&app));

    unreachable!("Garbage collection routines should run forever.");
}

//  TODO: Change this interval to be more realistic for production.
/// How long to wait before to calling [`bubbel_bath::AuthState::collect_garbage`].
const AUTH_COLLECT_GARBAGE_INTERVAL: Duration = Duration::from_secs(600);
async fn collect_auth_garbage(app: &Arc<AppState>) {
    loop {
        tokio::time::sleep(AUTH_COLLECT_GARBAGE_INTERVAL).await;
        debug!("Auth garbage collection in progress.");
        {
            let mut auth = app.inner.auth.write().await;
            auth.collect_garbage();
        }
        debug!("Auth garbage collection complete.");
    }
}

//  TODO: Change this interval to be more realistic for production.
/// How long to wait before to calling [`bubbel_bath::AuthState::collect_garbage`].
const ACC_LIMBO_COLLECT_GARBAGE_INTERVAL: Duration = Duration::from_secs(600);
async fn collect_acc_limbo_garbage(app: &Arc<AppState>) {
    loop {
        tokio::time::sleep(ACC_LIMBO_COLLECT_GARBAGE_INTERVAL).await;
        debug!("Account limbo garbage collection in progress.");
        {
            let mut db = app.inner.db.spawn();
            let mut acc_limbo = app.inner.acc_limbo.lock().await;
            acc_limbo.collect_garbage(&mut db);
        }
        debug!("Account limbo garbage collection complete.");
    }
}
