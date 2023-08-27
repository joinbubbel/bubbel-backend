use super::*;

mod on_delete;
mod on_edit;
mod on_new;

pub use on_delete::DataChannelOnDelete;
pub use on_edit::DataChannelOnEdit;
pub use on_new::DataChannelOnNew;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct DataChannelResponse {
    pub res: Option<DataChannelResponseType>,
    pub error: Option<DataChannelError>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum DataChannelResponseType {
    OnNew(DataChannelOnNew),
    OnEdit(DataChannelOnEdit),
    OnDelete(DataChannelOnDelete),
}
