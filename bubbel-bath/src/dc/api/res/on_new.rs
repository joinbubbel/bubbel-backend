use super::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DataChannelOnNew {
    pub item: DataChannelItem,
}
