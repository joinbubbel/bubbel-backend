use super::*;
use async_trait::async_trait;

pub fn configure_routes_with_router(
    mut router: Router<Arc<AppState>>,
    codegen_ctx: &mut CodegenContext,
) -> Router<Arc<AppState>> {
    //  This will be duplicated.
    //  add_codegen_ty!(codegen_ctx, DataChannelItem);

    add_codegen_ty!(codegen_ctx, DataChannelInitRequest);
    add_codegen_ty!(codegen_ctx, DataChannelInitResponse);
    add_codegen_ty!(codegen_ctx, DataChannelRequest);
    add_codegen_ty!(codegen_ctx, DataChannelResponse);

    router = router.route("/api/dc", get(ws_handler));
    router
}

pub struct WebSocketSendRecv(WebSocket);

#[async_trait]
impl SendRecv for WebSocketSendRecv {
    async fn send(&mut self, res: DataChannelResponse) {
        self.0
            .send(Message::Text(serde_json::to_string(&res).unwrap()))
            .await
            .unwrap();
    }
    async fn send_init(&mut self, res: DataChannelInitResponse) {
        self.0
            .send(Message::Text(serde_json::to_string(&res).unwrap()))
            .await
            .unwrap();
    }
    async fn recv(&mut self) -> Option<DataChannelRequest> {
        self.0.recv().await.and_then(|res| match res.unwrap() {
            Message::Text(text) => serde_json::from_str(&text).ok(),
            _ => None,
        })
    }
}

async fn ws_handler(State(state): State<Arc<AppState>>, ws: WebSocketUpgrade) -> impl IntoResponse {
    let state = Arc::clone(&state);
    ws.on_upgrade(move |socket| handle_socket(state, socket))
}

async fn handle_socket(state: Arc<AppState>, mut socket: WebSocket) {
    let init_req = socket.recv().await.unwrap().unwrap();
    let text = match init_req {
        Message::Text(text) => text,
        Message::Close(_) => return,
        _ => panic!("Expected string."),
    };
    let init_req = serde_json::from_str(&text).unwrap();
    dc_run(&state.inner, init_req, &mut WebSocketSendRecv(socket)).await
}
