use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct DataChannelOnDelete {
    pub chunk: DataChunkIndex,
    pub index: usize,
}
