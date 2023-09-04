use super::*;
use dumpster_axum::{
    InUploadLooseBase64, ResUploadLooseBase64,
    UploadLooseBase64Error as DumpsterUploadLooseBase64Error,
};
use reqwest::Client;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct UploadLooseBase64 {
    pub token: UserToken,
    pub file_name: String,
    pub data: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct UploadLooseBase64Out {
    pub object_name: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum UploadLooseBase64Error {
    NoAuth,
    InvalidBase64,
    DataConstraint { data_constraint_error: String },
    Internal { ierror: String },
}

pub async fn upload_loose_base64(
    auth: &AuthState,
    req: UploadLooseBase64,
) -> Result<UploadLooseBase64Out, UploadLooseBase64Error> {
    let Some(_) = auth.check_user_with_token(&req.token) else {
        Err(UploadLooseBase64Error::NoAuth)?
    };

    //  TODO Check User Dumpster Limits.

    let client = Client::new();
    let req_body = serde_json::to_string(&InUploadLooseBase64 {
        base64_data: req.data,
        file_name: req.file_name,
    })
    .unwrap();

    let res: ResUploadLooseBase64 = client
        .post("http://localhost:5757/upload_loose_base64")
        .body(req_body)
        .send()
        .await
        .map_err(|_| UploadLooseBase64Error::Internal {
            ierror: "Internal dumpster is offline.".to_owned(),
        })?
        .json()
        .await
        .map_err(|_| UploadLooseBase64Error::Internal {
            ierror: "Internal serialization error.".to_owned(),
        })?;

    if let Some(e) = res.error {
        Err(match e {
            DumpsterUploadLooseBase64Error::DataConstraint { reason } => {
                UploadLooseBase64Error::DataConstraint {
                    data_constraint_error: reason,
                }
            }
            DumpsterUploadLooseBase64Error::InvalidBase64 => UploadLooseBase64Error::InvalidBase64,
        })?;
    }

    let object_name = res.object_name.ok_or(UploadLooseBase64Error::Internal {
        ierror: "Internal impossible response.".to_owned(),
    })?;

    Ok(UploadLooseBase64Out { object_name })
}
