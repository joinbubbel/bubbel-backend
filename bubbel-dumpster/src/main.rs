use dumpster_axum::*;
use libdumpster::*;
use std::{net::SocketAddr, sync::Arc};

#[tokio::main]
async fn main() {
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
                .add_step(ImageOperationStep::MaxSize(1200, 200))
                .build(),
        ))
        .store("banner1200x200.jpeg")
        .build();

    let mount_dir = "/bubbel/dumpster";
    let fs = tokio_fs::TokioFileSystem::mount(mount_dir).await.unwrap();
    let exec = Executor::new(fs, &[profile_picture, banner_picture], &[]).await;
    let addr = SocketAddr::from(([0, 0, 0, 0], 5757));
    run_with_axum(exec, mount_dir, addr).await;
}
