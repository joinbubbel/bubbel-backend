use super::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct DataChannelCommandDelete {
    chunk: usize,
    index: usize,
}
