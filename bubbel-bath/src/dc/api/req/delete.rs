use super::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DataChannelCommandDelete {
    chunk: usize,
    index: usize,
}
