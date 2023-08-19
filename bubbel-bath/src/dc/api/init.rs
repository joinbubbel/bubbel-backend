use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct DataChannelInitRequest {
    pub token: UserToken,
    pub channel: DataChannelId,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct DataChannelInitResponse {
    pub current_chunk: Option<DataChunkId>,
    pub error: Option<DataChannelInitError>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum DataChannelInitError {
    NoAuth,
    ChannelNotFound,
    Internal { ierror: String },
}

/// Assumes that the channel is locked prior to running.
/// Assumes that data channel starts with one chunk.
pub(super) fn dc_init(
    db: &mut DataStateInstance,
    auth: &AuthState,
    req: &DataChannelInitRequest,
) -> Result<DataChunkId, DataChannelInitError> {
    auth.check_user_with_token(&req.token)
        .ok_or(DataChannelInitError::NoAuth)?;
    DataChannel::get(db, &req.channel)
        .map_err(|e| DataChannelInitError::Internal {
            ierror: e.to_string(),
        })?
        .ok_or(DataChannelInitError::ChannelNotFound)
        .map(|dc| DataChunkId((dc.chunks.len() - 1) as i32))
}
