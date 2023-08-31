use super::*;
use dumpster_axum::{InUpload, ResError, ResUpload};
use reqwest::Client;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct UploadBase64 {
    token: UserToken,
    class_name: String,
    data: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct UploadBase64Out {
    object_name: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum UploadBase64Error {
    NoAuth,
    InvalidBase64,
    DataRejected,
    Internal { ierror: String },
}

pub async fn upload_base64(
    auth: &AuthState,
    req: UploadBase64,
) -> Result<UploadBase64Out, UploadBase64Error> {
    let Some(_) = auth.check_user_with_token(&req.token) else {
        Err(UploadBase64Error::NoAuth)?
    };

    //  TODO Check User Dumpster Limits.

    let client = Client::new();
    let req_body = serde_json::to_string(&InUpload {
        base64_data: req.data,
        class_name: req.class_name,
    })
    .unwrap();

    let res: ResUpload = client
        .post("http://localhost:5757/upload_base64")
        .body(req_body)
        .send()
        .await
        .map_err(|_| UploadBase64Error::Internal {
            ierror: "Internal dumpster is offline.".to_owned(),
        })?
        .json()
        .await
        .map_err(|_| UploadBase64Error::Internal {
            ierror: "Internal serialization error.".to_owned(),
        })?;

    if let Some(e) = res.error {
        Err(match e {
            ResError::DataRejected => UploadBase64Error::DataRejected,
            ResError::InvalidBase64 => UploadBase64Error::InvalidBase64,
        })?;
    }

    let object_name = res.object_name.ok_or(UploadBase64Error::Internal {
        ierror: "Internal impossible response.".to_owned(),
    })?;

    Ok(UploadBase64Out { object_name })
}