use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetDataChannelChunk {
    token: UserToken,
    channel_id: DataChannelId,
    chunk_index: DataChunkIndex,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct GetDataChannelChunkOut {
    chunk: DataChunk,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum GetDataChannelChunkError {
    NoAuth,
    ChunkNotFound,
    ChannelNotFound,
    Internal { ierror: String },
}

pub fn get_data_channel_chunk(
    db: &mut DataStateInstance,
    auth: &AuthState,
    req: GetDataChannelChunk,
) -> Result<GetDataChannelChunkOut, GetDataChannelChunkError> {
    auth.check_user_with_token(&req.token)
        .ok_or(GetDataChannelChunkError::NoAuth)?;
    let channel = DataChannel::get(db, &req.channel_id)
        .map_err(|e| GetDataChannelChunkError::Internal {
            ierror: e.to_string(),
        })?
        .ok_or(GetDataChannelChunkError::ChannelNotFound)?;
    let chunk_id = channel
        .chunks
        .get(req.chunk_index.0)
        .ok_or(GetDataChannelChunkError::ChunkNotFound)?;
    let chunk = DataChunk::get(db, chunk_id)
        .map_err(|e| GetDataChannelChunkError::Internal {
            ierror: e.to_string(),
        })?
        .ok_or(GetDataChannelChunkError::ChunkNotFound)?;
    Ok(GetDataChannelChunkOut { chunk })
}
