use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct DataChannelOnEdit {
    pub chunk: DataChunkIndex,
    pub index: usize,
    pub new_item: DataChannelItem,
}
