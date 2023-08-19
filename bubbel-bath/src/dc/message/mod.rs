use super::*;

mod text;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Message {
    Text(Text),
}

pub use text::Text;
