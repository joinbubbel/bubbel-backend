use super::*;

mod delete;
mod send;

pub use send::dc_send;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct DataChannelRequest {
    pub token: UserToken,
    pub command: DataChannelCommandType,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum DataChannelCommandType {
    Send(send::DataChannelCommandSend),
    Delete(delete::DataChannelCommandDelete),
}
