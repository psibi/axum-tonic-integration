mod axum_app;
mod grpc;

use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let axum_app = axum_app::http_app();

    let grpc_service = grpc::EchoService::default();

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(axum_app)
        .await
        .unwrap();
    Ok(())
}
