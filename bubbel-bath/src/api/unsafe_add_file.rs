use super::*;

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
    Internal { ierror: String },
}

pub fn unsafe_add_file(req: UnsafeAddFile) -> Result<UnsafeAddFileOut, UnsafeAddFileError> {
    let token = generate_token_alphanumeric(32);
    let write_dir = "/tmp/unsafe_data/".to_owned() + &token + &req.extension;
    std::fs::write(write_dir, req.data.as_bytes()).map_err(|e| UnsafeAddFileError::Internal {
        ierror: e.to_string(),
    })?;
    Ok(UnsafeAddFileOut {
        file_link: format!(
            "https://api.joinbubbel.com/unsafe_data/{}{}",
            token, req.extension
        ),
    })
}
