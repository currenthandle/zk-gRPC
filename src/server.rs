use tonic::{transport::Server, Request, Response, Status};

pub mod zkp_auth {
    include!("./zkp_auth.rs");
}

use zkp_auth::auth_server::{Auth, AuthServer};
use zkp_auth::{
    AuthenicationAnswerRequest, AuthenicationAnswerResponse, AuthenticationChallengeRequest,
    AuthenticationChallengeResponse, RegisterRequest, RegisterResponse,
};

#[derive(Debug, Default)]
pub struct AuthImpl {}

#[tonic::async_trait]
impl Auth for AuthImpl {
    async fn register(
        &self,
        request: Request<RegisterRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<RegisterResponse>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        // let reply = hello_world::HelloReply {
        //     message: format!("Hello {}!", request.into_inner().name).into(), // We must use .into_inner() as the fields of gRPC requests and responses are private
        // };

        Ok(Response::new(RegisterResponse {}))
    }
    async fn create_authenication_challenge(
        &self,
        request: tonic::Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        todo!()
    }
    async fn verify_authentication(
        &self,
        request: Request<AuthenicationAnswerRequest>,
    ) -> Result<Response<AuthenicationAnswerResponse>, Status> {
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running the server");
    // let addr = "[::1]:50051".parse()?;
    let addr = "127.0.0.1:50051".parse()?;
    let auth_impl = AuthImpl::default();

    Server::builder()
        .add_service(AuthServer::new(auth_impl))
        .serve(addr)
        .await?;

    Ok(())
}
