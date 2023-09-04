use super::*;
use b64::ToBase64;
use url::Url;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct ResolveAndUpload {
    token: UserToken,
    url: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct ResolveAndUploadOut {
    object_name: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum ResolveAndUploadError {
    FetchFailed { fetch_error: String },
    FetchBytesFailed { fetch_bytes_error: String },
    CannotGetUrlFileName,
    ConvertToBase64Failed,
    NoAuth,
    DataConstraint { data_constraint_error: String },
    Internal { ierror: String },
}

pub async fn resolve_and_upload(
    auth: &AuthState,
    req: ResolveAndUpload,
) -> Result<ResolveAndUploadOut, ResolveAndUploadError> {
    let Some(_) = auth.check_user_with_token(&req.token) else {
        Err(ResolveAndUploadError::NoAuth)?
    };

    let data = reqwest::get(&req.url)
        .await
        .map_err(|e| ResolveAndUploadError::FetchFailed {
            fetch_error: e.to_string(),
        })?
        .bytes()
        .await
        .map_err(|e| ResolveAndUploadError::FetchBytesFailed {
            fetch_bytes_error: e.to_string(),
        })?
        .to_base64(b64::STANDARD);

    let file_name =
        extract_filename(&req.url).ok_or(ResolveAndUploadError::CannotGetUrlFileName)?;

    let res = upload_loose_base64(
        auth,
        UploadLooseBase64 {
            token: req.token,
            file_name,
            data,
        },
    )
    .await
    .map_err(|e| match e {
        UploadLooseBase64Error::NoAuth => ResolveAndUploadError::NoAuth,
        UploadLooseBase64Error::DataConstraint {
            data_constraint_error,
        } => ResolveAndUploadError::DataConstraint {
            data_constraint_error,
        },
        UploadLooseBase64Error::Internal { ierror } => ResolveAndUploadError::Internal { ierror },
        UploadLooseBase64Error::InvalidBase64 => ResolveAndUploadError::Internal {
            ierror: "Internal Invalid Base64".to_owned(),
        },
    })?;

    Ok(ResolveAndUploadOut {
        object_name: res.object_name,
    })
}

fn extract_filename(url_str: &str) -> Option<String> {
    if let Ok(url) = Url::parse(url_str) {
        if let Some(mut path_segments) = url.path_segments() {
            if let Some(filename) = path_segments.next_back() {
                if let Some(pos) = filename.find(|c| c == '?' || c == '#') {
                    return Some(filename[..pos].to_string());
                } else {
                    return Some(filename.to_string());
                }
            }
        }
    }
    None
}
