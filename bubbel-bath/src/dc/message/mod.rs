use super::*;

mod text;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum Message {
    Text(Text),
}

pub use text::Text;
