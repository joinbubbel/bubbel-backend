use super::*;

mod on_new;

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
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum DataChannelError {
    NoAuth,
    ChannelNotFound,
    Internal { ierror: String },
}
