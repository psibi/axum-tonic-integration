use echo::echo_server::{Echo, EchoServer};
use echo::{EchoRequest, EchoResponse};
use tonic::{transport::Server,Status, Request, Response};
use tonic::transport::server::{Router, RouterService};
use tonic::transport;
use tower::util::BoxCloneService;
use tower::BoxError;

pub mod echo {
    tonic::include_proto!("echo");
}

#[derive(Debug, Default)]
pub struct EchoService();

#[tonic::async_trait]
impl Echo for EchoService {
    async fn reply_echo(
        &self,
        request: Request<EchoRequest>,
    ) -> Result<Response<EchoResponse>, Status> {
        println!("Request: {:?}", request);
        Ok(Response::new(EchoResponse {
            message: request.into_inner().message,
        }))
    }
}


// pub fn grpc_service() -> BoxCloneService<Request,Response, BoxError> {
//     let service = EchoService::default();
//     let server = Server::builder().add_service(EchoServer::new(service)).into_service();
//     BoxCloneService::new(server)
// }
