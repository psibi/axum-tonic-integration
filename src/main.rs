use std::net::SocketAddr;

mod axum_app;
mod grpc;
mod hybrid;

use tonic::transport::Server;
use hybrid::hybrid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let axum_service = axum_app::http_app();

    let echo_service = grpc::EchoService::default();
    let grpc_echo_server = grpc::echo::echo_server::EchoServer::new(echo_service);

    let grpc_service = Server::builder()
        .add_service(grpc_echo_server)
        .into_service();

    let hybrid_make_service = hybrid(axum_service, grpc_service);
    let server = hyper::Server::bind(&addr).serve(hybrid_make_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}
