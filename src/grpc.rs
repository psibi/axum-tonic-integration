use echo::echo_server::Echo;
use echo::{EchoRequest, EchoResponse};
use tonic::{Request, Response, Status};

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
