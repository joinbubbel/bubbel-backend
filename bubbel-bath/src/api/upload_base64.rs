use super::*;
use b64::FromBase64;
use libdumpster::*;
use std::{fs, sync::Arc};
// use dumpster_axum::{
//     InUploadBase64, ResUploadBase64, UploadBase64Error as DumpsterUploadBase64Error,
// };
// use reqwest::Client;

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
    DataCorrupt { data_corrupt_error: String },
    DataConstraint { data_constraint_error: String },
    Internal { ierror: String },
}

pub async fn upload_base64(
    auth: &AuthState,
    req: UploadBase64,
) -> Result<UploadBase64Out, UploadBase64Error> {
    //  TODO

    let mount_dir = "/bubbel/dumpster";
    fs::create_dir_all(mount_dir).unwrap();
    let fs = tokio_fs::TokioFileSystem::mount(mount_dir).await.unwrap();

    let profile_picture = Class::builder("profile_picture")
        .op(Arc::new(
            ImageOperation::builder(ImageFormat::Jpeg)
                .add_step(ImageOperationStep::Resize(150, 150))
                .build(),
        ))
        .store("pfp150x150.jpeg")
        .build();

    let banner_picture = Class::builder("banner_picture")
        .op(Arc::new(
            ImageOperation::builder(ImageFormat::Jpeg)
                .add_step(ImageOperationStep::Resize(1200, 200))
                .build(),
        ))
        .store("banner1200x200.jpeg")
        .build();
    let exec = Executor::new(fs, &[profile_picture, banner_picture], &[]).await;

    let (_, base64_data) = req.data.split_once(',').unwrap_or(("", &req.data));

    let object_name = exec
        .incoming(&req.class_name, base64_data.from_base64().unwrap())
        .await;

    let object_name = match object_name {
        Ok(object_name) => object_name,
        Err(e) => Err(match e {
            OperationReject::DataCorrupt { reason } => UploadBase64Error::DataCorrupt {
                data_corrupt_error: reason,
            },
            OperationReject::DataConstraint { reason } => UploadBase64Error::DataConstraint {
                data_constraint_error: reason,
            },
        })?,
    };

    Ok(UploadBase64Out { object_name })

    // let Some(_) = auth.check_user_with_token(&req.token) else {
    //     Err(UploadBase64Error::NoAuth)?
    // };

    // //  TODO Check User Dumpster Limits.

    // let client = Client::new();

    // let res: ResUploadBase64 = client
    //     .post("http://localhost:5757/upload_base64")
    //     .json(&InUploadBase64 {
    //         base64_data: req.data,
    //         class_name: req.class_name,
    //     })
    //     .send()
    //     .await
    //     .map_err(|_| UploadBase64Error::Internal {
    //         ierror: "Internal dumpster is offline.".to_owned(),
    //     })?
    //     .json()
    //     .await
    //     .map_err(|_| UploadBase64Error::Internal {
    //         ierror: "Internal serialization error.".to_owned(),
    //     })?;

    // if let Some(e) = res.error {
    //     Err(match e {
    //         DumpsterUploadBase64Error::DataCorrupt { reason } => UploadBase64Error::DataCorrupt {
    //             data_corrupt_error: reason,
    //         },
    //         DumpsterUploadBase64Error::DataConstraint { reason } => {
    //             UploadBase64Error::DataConstraint {
    //                 data_constraint_error: reason,
    //             }
    //         }
    //         DumpsterUploadBase64Error::InvalidBase64 => UploadBase64Error::InvalidBase64,
    //     })?;
    // }

    // let object_name = res.object_name.ok_or(UploadBase64Error::Internal {
    //     ierror: "Internal impossible response.".to_owned(),
    // })?;

    // Ok(UploadBase64Out { object_name })
}
