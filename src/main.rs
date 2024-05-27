use proto::admin_server::{Admin,AdminServer};
use proto::calculator_server::{Calculator,CalculatorServer};
use tonic::transport::Server;

mod proto {
    tonic::include_proto!("calculator");
    tonic::include_proto!("admin");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("calculator_descriptor");
}

#[derive(Debug, Default)]
struct CalculatorService;


#[tonic::async_trait]
impl Calculator for CalculatorService {
    async fn get_app_version(
        &self,
        _request: tonic::Request<proto::CalculatorEmptyRequest>,
    ) -> Result<tonic::Response<proto::CalculatorResponse>, tonic::Status> {
        let response = proto::CalculatorResponse {
            name: "VersionTen".to_string(),
        };
        Ok(tonic::Response::new(response))
    }
}

#[derive(Default, Debug)]
struct AdminService;

#[tonic::async_trait]
impl Admin for AdminService {
    async fn get_admin_user(
        &self,
        _request: tonic::Request<proto::AdminEmptyRequest>,
    ) -> Result<tonic::Response<proto::AdminResponse>, tonic::Status> {
        let response = proto::AdminResponse {
            name: "John".to_string(),
        };
        Ok(tonic::Response::new(response))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let calculator_service = CalculatorServer::new(CalculatorService::default());
    let admin_service = AdminServer::new(AdminService::default());

    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        .add_service(service)
        .add_service(calculator_service)
        .add_service(admin_service)
        .serve(addr)
        .await?;

    Ok(())
}
