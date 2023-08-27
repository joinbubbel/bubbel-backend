use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct DataChannelCommandEdit {
    chunk: DataChunkIndex,
    index: usize,
    new_message: Message,
}

pub async fn dc_edit(
    db: &mut DataStateInstance,
    auth: &AuthState,
    chs: &ChannelState,
    channel_id: &DataChannelId,
    req: &DataChannelRequest,
    cmd: &DataChannelCommandEdit,
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

    let item = if let Some(item) = item {
        if item.sender != sender {
            return Err(DataChannelError::NoAuth);
        }

        item.message = cmd.new_message.clone();
        item.edit_time = Some(UnixTime::now());

        item.clone()
    } else {
        return Err(DataChannelError::DataItemDeleted);
    };

    chunk
        .update(db, chunk_id)
        .map_err(|e| DataChannelError::Internal {
            ierror: e.to_string(),
        })?;

    chs.broadcast(
        channel_id,
        DataChannelResponse {
            res: Some(DataChannelResponseType::OnEdit(DataChannelOnEdit {
                chunk: cmd.chunk,
                index: cmd.index,
                new_item: item,
            })),
            error: None,
        },
    );

    Ok(())
}
