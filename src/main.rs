use std::{error::Error, net::SocketAddr};

use dotenv::dotenv;
use interceptor::api_key::ApiKeyInterceptor;
use middleware::logger::LoggerMiddlewareLayer;
use proto::calculator_server::CalculatorServer;
use service::calculator_service::CalculatorService;
use tonic::transport::Server;

pub(crate) mod proto {
    // Name of the package in the ".proto" file
    tonic::include_proto!("calculator");

    pub const FILE_DESCRIPTOR_SET: &[u8] =
        // set in the buid.rs file
        tonic::include_file_descriptor_set!("calculator_descriptor");
}

pub(crate) mod interceptor;
pub(crate) mod middleware;
pub(crate) mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    env_logger::init();

    let addr = "[::1]:4000".parse::<SocketAddr>()?;
    let calc = CalculatorService::default();
    let interceptor = ApiKeyInterceptor::default();

    // laod reflection
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    let layer =
        tower::ServiceBuilder::new().layer(LoggerMiddlewareLayer::default());

    log::info!("🚀 Server is running...");
    // lauch server
    Server::builder()
        .layer(layer)
        .add_service(reflection_service)
        .add_service(CalculatorServer::with_interceptor(calc, interceptor))
        .serve(addr)
        .await?;

    Ok(())
}
