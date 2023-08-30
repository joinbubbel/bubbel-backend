use super::*;
use b64::FromBase64;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct UnsafeAddFile {
    extension: String,
    data: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct UnsafeAddFileOut {
    file_link: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum UnsafeAddFileError {
    InvalidBase64 { base64_error: String },
    Internal { ierror: String },
}

pub async fn unsafe_add_file(req: UnsafeAddFile) -> Result<UnsafeAddFileOut, UnsafeAddFileError> {
    let token = generate_token_alphanumeric(32);
    let write_dir = "/tmp/unsafe_data/".to_owned() + &token + &req.extension;
    let data = req
        .data
        .from_base64()
        .map_err(|e| UnsafeAddFileError::InvalidBase64 {
            base64_error: e.to_string(),
        })?;
    std::fs::write(write_dir, data).map_err(|e| UnsafeAddFileError::Internal {
        ierror: e.to_string(),
    })?;
    Ok(UnsafeAddFileOut {
        file_link: format!(
            "https://api.joinbubbel.com/unsafe_data/{}{}",
            token, req.extension
        ),
    })
}
