use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct DataChannelCommandSend {
    message: Message,
}

pub async fn dc_send(
    db: &mut DataStateInstance,
    auth: &AuthState,
    chs: &ChannelState,
    channel_id: &DataChannelId,
    req: &DataChannelRequest,
    cmd: &DataChannelCommandSend,
) -> Result<(), DataChannelError> {
    let sender = auth
        .check_user_with_token(&req.token)
        .ok_or(DataChannelError::NoAuth)?;

    let channel_lock = chs.get_channel_lock(channel_id);
    let _guard = channel_lock.lock().await;
    let mut channel = DataChannel::get(db, channel_id)
        .map_err(|e| DataChannelError::Internal {
            ierror: e.to_string(),
        })?
        .ok_or(DataChannelError::ChannelNotFound)?;

    let item = DataChannelItem {
        sender,
        post_time: UnixTime::now(),
        edit_time: None,
        message: cmd.message.clone(),
    };

    let (mut chunk, chunk_id) = if let Some(last_chunk_id) = channel.chunks.last() {
        let chunk = DataChunk::get(db, last_chunk_id)
            .map_err(|e| DataChannelError::Internal {
                ierror: e.to_string(),
            })?
            .unwrap();
        if chunk.items.len() == DATA_CHUNK_MAX_COUNT {
            channel
                .push_chunk(db)
                .map_err(|e| DataChannelError::Internal {
                    ierror: e.to_string(),
                })?
        } else {
            (chunk, *last_chunk_id)
        }
    } else {
        channel
            .push_chunk(db)
            .map_err(|e| DataChannelError::Internal {
                ierror: e.to_string(),
            })?
    };
    chunk.items.push(Some(item.clone()));
    chunk
        .update(db, &chunk_id)
        .map_err(|e| DataChannelError::Internal {
            ierror: e.to_string(),
        })?;
    channel
        .update(db, channel_id)
        .map_err(|e| DataChannelError::Internal {
            ierror: e.to_string(),
        })?;

    chs.broadcast(
        channel_id,
        DataChannelResponse {
            res: Some(DataChannelResponseType::OnNew(DataChannelOnNew {
                item,
                chunk: DataChunkIndex(channel.chunks.len() - 1),
                index: chunk.items.len() - 1,
            })),
            error: None,
        },
    );

    Ok(())
}
