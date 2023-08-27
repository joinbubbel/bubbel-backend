use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct DataChannelOnNew {
    pub item: DataChannelItem,
    pub chunk: DataChunkIndex,
    pub index: usize,
}
