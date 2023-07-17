use super::*;
use serde::{Deserialize, Serialize};

const DEBUG_STATE_BUF_COUNT: usize = 15;

#[derive(Serialize, Deserialize, Clone)]
pub struct DebugState {
    enabled: bool,
    incoming: [Option<(u64, String)>; DEBUG_STATE_BUF_COUNT],
    outgoing: [Option<(u64, String)>; DEBUG_STATE_BUF_COUNT],

    #[serde(skip)]
    incoming_current: usize,
    #[serde(skip)]
    outgoing_current: usize,
    #[serde(skip)]
    password: String,
    #[serde(skip)]
    debug_id: u64,
}

impl DebugState {
    pub fn new(enabled: bool, password: Option<String>) -> Self {
        Self {
            enabled,
            incoming: (0..DEBUG_STATE_BUF_COUNT)
                .map(|_| None)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            outgoing: (0..DEBUG_STATE_BUF_COUNT)
                .map(|_| None)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),

            incoming_current: 0,
            outgoing_current: 0,
            password: if enabled {
                password.unwrap()
            } else {
                String::new()
            },
            debug_id: 0,
        }
    }

    pub fn push_incoming<T: Serialize>(&mut self, data: &T) {
        if !self.enabled {
            return;
        }
        let s = serde_json::to_string_pretty(data).unwrap();
        self.incoming[self.incoming_current] = Some((self.debug_id, s));
        self.incoming_current = (self.incoming_current + 1) % DEBUG_STATE_BUF_COUNT;
        self.debug_id += 1;
    }

    pub fn push_outgoing<T: Serialize>(&mut self, data: &T) {
        if !self.enabled {
            return;
        }
        let s = serde_json::to_string_pretty(data).unwrap();
        self.outgoing[self.outgoing_current] = Some((self.debug_id, s));
        self.outgoing_current = (self.outgoing_current + 1) % DEBUG_STATE_BUF_COUNT;
        self.debug_id += 1;
    }
}

#[derive(Serialize, Deserialize)]
pub struct InDebug {
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResDebug {
    error: Option<i32>,
    #[serde(flatten)]
    out: Option<DebugState>,
}

pub async fn api_debug(
    State(state): State<Arc<AppState>>,
    Json(req): Json<InDebug>,
) -> Json<ResDebug> {
    let debug = state.debug.read().unwrap();

    eprintln!("{} {} {}", req.password, debug.enabled, debug.password);
    if req.password == debug.password && debug.enabled {
        Json(ResDebug {
            error: None,
            out: Some(debug.clone()),
        })
    } else {
        Json(ResDebug {
            error: Some(1),
            out: None,
        })
    }
}
