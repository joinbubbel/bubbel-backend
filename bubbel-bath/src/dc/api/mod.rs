use super::*;
use async_trait::async_trait;

mod init;
mod req;
mod res;

pub use init::*;
pub use req::*;
pub use res::*;

#[async_trait]
pub trait SendRecv {
    async fn send(&mut self, res: DataChannelResponse);
    async fn send_init(&mut self, res: DataChannelInitResponse);
    async fn recv(&mut self) -> Option<DataChannelRequest>;
}

pub async fn dc_run(
    state: &InnerState,
    init_req: DataChannelInitRequest,
    socket: &mut impl SendRecv,
) {
    let mut rx = {
        let mut db = state.db.spawn();
        let auth = state.auth.read().await;
        let chs_lock = state.chs.get_channel_lock(&init_req.channel);
        let _chs_guard = chs_lock.lock().await;
        let dc_init_res = dc_init(&mut db, &auth, &init_req);
        let current_chunk = match dc_init_res {
            Ok(current_chunk) => Some(current_chunk),
            Err(e) => {
                socket
                    .send_init(DataChannelInitResponse {
                        current_chunk: None,
                        error: Some(e),
                    })
                    .await;
                return;
            }
        };
        socket
            .send_init(DataChannelInitResponse {
                current_chunk,
                error: None,
            })
            .await;
        state.chs.insert_listener(&init_req.channel)
    };

    loop {
        tokio::select! {
            req = socket.recv() => {
                if let Some(req) = req {
                    if let Err(e) = handle_recv(state, &init_req.channel, req).await {
                        socket.send(DataChannelResponse {
                            res: None,
                            error: Some(e),
                        }).await;
                    }
                } else {
                    break;
                }
            }
            Ok(msg) = rx.recv() => {
                socket.send(msg).await
            }
        }
    }

    //  TODO Clean up.
}

pub async fn handle_recv(
    state: &InnerState,
    channel: &DataChannelId,
    req: DataChannelRequest,
) -> Result<(), DataChannelError> {
    match &req.command {
        DataChannelCommandType::Send(cmd) => {
            let mut db = state.db.spawn();
            let auth = state.auth.read().await;
            dc_send(&mut db, &auth, &state.chs, channel, &req, cmd).await
        }
        DataChannelCommandType::Delete(_) => todo!(),
    }
}
