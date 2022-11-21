mod tf;

use tf::infer;
use base64::decode;
use once_cell::sync::Lazy;
use tonic::{transport::Server, Request, Response, Status};
use general::{GeneralRequest, GeneralResponse};
use general::general_project_server::{GeneralProject, GeneralProjectServer};

pub mod general {
    tonic::include_proto!("general");
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("general_descriptor");
}

static MODEL: Lazy<&[u8]> = Lazy::new(|| {
    let model = include_bytes!("../models/model.pb");
    model
});

#[derive(Default)]
pub struct MyProject {}

#[tonic::async_trait]
impl GeneralProject for MyProject {
    async fn infer_request(
        &self,
        request: Request<GeneralRequest>,
    ) -> Result<Response<GeneralResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());
        let image_bytes = decode(request.into_inner().data).unwrap();
        let message = infer(&image_bytes).ok().ok_or(Status::new(tonic::Code::Internal, "Infer error"))?;
        let reply = GeneralResponse {
            message,
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();

    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(general::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();
    let addr = "0.0.0.0:50051".parse().unwrap();
    let project = MyProject::default();

    println!("ProjectServer listening on {}", addr);

    Server::builder()
        .add_service(service)
        .add_service(GeneralProjectServer::new(project))
        .serve(addr)
        .await?;

    Ok(())
}
