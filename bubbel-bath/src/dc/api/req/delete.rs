use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct DataChannelCommandDelete {
    chunk: DataChunkIndex,
    index: usize,
}

pub async fn dc_delete(
    db: &mut DataStateInstance,
    auth: &AuthState,
    chs: &ChannelState,
    channel_id: &DataChannelId,
    req: &DataChannelRequest,
    cmd: &DataChannelCommandDelete,
) -> Result<(), DataChannelError> {
    let sender = auth
        .check_user_with_token(&req.token)
        .ok_or(DataChannelError::NoAuth)?;
    let channel_lock = chs.get_channel_lock(channel_id);
    let _guard = channel_lock.lock().await;
    let channel = DataChannel::get(db, channel_id)
        .map_err(|e| DataChannelError::Internal {
            ierror: e.to_string(),
        })?
        .ok_or(DataChannelError::ChannelNotFound)?;
    let chunk_id = channel
        .chunks
        .get(cmd.chunk.0)
        .ok_or(DataChannelError::ChunkNotFound)?;
    let mut chunk = DataChunk::get(db, chunk_id)
        .map_err(|e| DataChannelError::Internal {
            ierror: e.to_string(),
        })?
        .ok_or(DataChannelError::ChunkNotFound)?;
    let item = chunk
        .items
        .get_mut(cmd.index)
        .ok_or(DataChannelError::DataItemNotFound)?;

    if let Some(item_inner) = item {
        if item_inner.sender != sender {
            return Err(DataChannelError::NoAuth);
        }
    } else {
        return Err(DataChannelError::DataItemDeleted);
    }

    *item = None;

    chunk
        .update(db, chunk_id)
        .map_err(|e| DataChannelError::Internal {
            ierror: e.to_string(),
        })?;

    chs.broadcast(
        channel_id,
        DataChannelResponse {
            res: Some(DataChannelResponseType::OnDelete(DataChannelOnDelete {
                chunk: cmd.chunk,
                index: cmd.index,
            })),
            error: None,
        },
    );

    Ok(())
}
