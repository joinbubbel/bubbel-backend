use super::*;
use std::time::SystemTime;

mod api;
mod message;

pub use api::*;
pub use message::Message;

const DATA_CHUNK_MAX_COUNT: usize = 128;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct DataChunkId(i32);

#[derive(Serialize, Deserialize, Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct DataChannelId(i32);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UnixTime(u64);

impl UnixTime {
    pub fn now() -> Self {
        Self(
            std::time::SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct DataChannel {
    chunks: Vec<DataChunkId>,
}

impl DataChannel {
    /// Always creates one empty chunk.
    pub fn insert_new(
        db: &mut DataStateInstance,
    ) -> Result<(DataChannel, DataChannelId), DatabaseError> {
        use crate::schema::data_channels::dsl;

        let mut dc = DataChannel::default();
        dc.push_chunk(db)?;

        diesel::insert_into(dsl::data_channels)
            .values(dsl::data_channel.eq(serde_json::to_string(&dc).unwrap()))
            .returning(dsl::id)
            .get_result::<i32>(&mut db.db)
            .map(|id| (dc, DataChannelId(id)))
            .map_err(DatabaseError::from)
    }

    pub fn get(
        db: &mut DataStateInstance,
        id: &DataChannelId,
    ) -> Result<Option<Self>, DatabaseError> {
        use crate::schema::data_channels::dsl;

        let data_channel = dsl::data_channels
            .select(dsl::data_channel)
            .filter(dsl::id.eq(id.0))
            .load::<String>(&mut db.db)
            .map(|v| v.first().cloned())
            .map_err(DatabaseError::from)?;
        Ok(data_channel.map(|data_channel| serde_json::from_str(&data_channel).unwrap()))
    }

    pub fn push_chunk(
        &mut self,
        db: &mut DataStateInstance,
    ) -> Result<(DataChunk, DataChunkId), DatabaseError> {
        let (chunk, chunk_id) = DataChunk::insert_new(db)?;
        self.chunks.push(chunk_id);
        Ok((chunk, chunk_id))
    }

    pub fn update(
        &self,
        db: &mut DataStateInstance,
        id: &DataChannelId,
    ) -> Result<(), DatabaseError> {
        use crate::schema::data_channels::dsl;

        diesel::update(dsl::data_channels)
            .set(dsl::data_channel.eq(serde_json::to_string(self).unwrap()))
            .filter(dsl::id.eq(id.0))
            .execute(&mut db.db)
            .map(|_| ())
            .map_err(DatabaseError::from)
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct DataChunk {
    items: Vec<Option<DataChannelItem>>,
}

impl DataChunk {
    pub fn insert_new(
        db: &mut DataStateInstance,
    ) -> Result<(DataChunk, DataChunkId), DatabaseError> {
        use crate::schema::data_chunks::dsl;

        let chunk = DataChunk::default();
        diesel::insert_into(dsl::data_chunks)
            .values(dsl::data_chunk.eq(serde_json::to_string(&chunk).unwrap()))
            .returning(dsl::id)
            .get_result::<i32>(&mut db.db)
            .map(|id| (chunk, DataChunkId(id)))
            .map_err(DatabaseError::from)
    }

    pub fn get(
        db: &mut DataStateInstance,
        id: &DataChunkId,
    ) -> Result<Option<Self>, DatabaseError> {
        use crate::schema::data_chunks::dsl;

        let data_chunk = dsl::data_chunks
            .select(dsl::data_chunk)
            .filter(dsl::id.eq(id.0))
            .load::<String>(&mut db.db)
            .map(|v| v.first().cloned())
            .map_err(DatabaseError::from)?;
        Ok(data_chunk.map(|data_chunk| serde_json::from_str(&data_chunk).unwrap()))
    }

    pub fn update(
        &self,
        db: &mut DataStateInstance,
        id: &DataChunkId,
    ) -> Result<(), DatabaseError> {
        use crate::schema::data_chunks::dsl;

        diesel::update(dsl::data_chunks)
            .set(dsl::data_chunk.eq(serde_json::to_string(self).unwrap()))
            .filter(dsl::id.eq(id.0))
            .execute(&mut db.db)
            .map(|_| ())
            .map_err(DatabaseError::from)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DataChannelItem {
    sender: UserId,
    post_time: UnixTime,
    edit_time: Option<UnixTime>,
    message: Message,
}
