use super::*;

mod on_new;

pub use on_new::DataChannelOnNew;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DataChannelResponse {
    pub res: Option<DataChannelResponseType>,
    pub error: Option<DataChannelError>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum DataChannelResponseType {
    OnNew(DataChannelOnNew),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum DataChannelError {
    NoAuth,
    ChannelNotFound,
    Internal { ierror: String },
}
