use std::collections::HashMap;
use std::sync::Mutex;
use tonic::{transport::Server, Code, Request, Response, Status};
use zkp_course::{random_number, random_string, verify, G, H, P};

pub mod zkp_auth {
    include!("./zkp_auth.rs");
}

use zkp_auth::auth_server::{Auth, AuthServer};
use zkp_auth::{
    AuthenicationAnswerRequest, AuthenicationAnswerResponse, AuthenticationChallengeRequest,
    AuthenticationChallengeResponse, RegisterRequest, RegisterResponse,
};

#[derive(Debug, Default)]
pub struct UserInfo {
    // pub user: String,
    pub y1: u32,
    pub y2: u32,
    pub r1: u32,
    pub r2: u32,
    pub c: u32,
    pub session_id: String,
}

#[derive(Debug, Default)]
pub struct AuthImpl {
    pub user_info: Mutex<HashMap<String, UserInfo>>, // <user_id, UserInfo>
    pub auth_info: Mutex<HashMap<String, String>>,   // <auth_id, user_id>
}

#[tonic::async_trait]
impl Auth for AuthImpl {
    async fn register(
        &self,
        request: Request<RegisterRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<RegisterResponse>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let request = request.into_inner();
        let user_id = request.user;

        let user_info = UserInfo {
            y1: request.y1,
            y2: request.y2,
            r1: 0,
            r2: 0,
            c: 0,
            session_id: String::new(),
        };

        let user_info_hashmap = &mut self.user_info.lock().unwrap(); //TODO : inprove unwrap
        user_info_hashmap.insert(user_id, user_info);

        println!("user_info_hashmap: {:?}", user_info_hashmap);

        Ok(Response::new(RegisterResponse {}))
    }
    async fn create_authenication_challenge(
        &self,
        request: tonic::Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        println!("Got AuthenticationChallengeRequest request: {:?}", request);

        let request = request.into_inner();
        let user_id = request.user;

        let auth_id = String::new();
        let c = 0u32;

        let user_info_hashmap = &mut self.user_info.lock().unwrap(); //TODO : inprove unwrap
        if let Some(user_info) = user_info_hashmap.get_mut(&user_id) {
            let auth_id = random_string(6);
            let c = random_number() % 10;

            user_info.r1 = request.r1;
            user_info.r2 = request.r2;
            user_info.c = c;

            println!("");
            println!("user_info_hashmap: {:?}", user_info_hashmap);
            let auth_info_hashmap = &mut self.auth_info.lock().unwrap(); //TODO : inprove unwrap
            auth_info_hashmap.insert(auth_id.clone(), user_id);

            Ok(Response::new(AuthenticationChallengeResponse {
                auth_id,
                c,
            }))
        } else {
            Err(Status::new(
                Code::NotFound,
                format!("User {} not found", user_id),
            ))
        }
    }
    async fn verify_authentication(
        &self,
        request: Request<AuthenicationAnswerRequest>,
    ) -> Result<Response<AuthenicationAnswerResponse>, Status> {
        let request = request.into_inner();
        let auth_id = request.auth_id;

        let auth_info_hashmap = &self.auth_info.lock().unwrap(); //TODO : inprove unwrap

        if let Some(user_id) = auth_info_hashmap.get(&auth_id) {
            let user_info_hashmap = &mut self.user_info.lock().unwrap(); //TODO : inprove unwrap

            if let Some(user_info) = user_info_hashmap.get_mut(user_id) {
                let s = request.s;
                let y1 = user_info.y1;
                let y2 = user_info.y2;
                let r1 = user_info.r1;
                let r2 = user_info.r2;
                let c = user_info.c;

                if verify(G, H, P, y1, y2, r1, r2, c, s) {
                    let session_id = random_string(6);
                    user_info.session_id = session_id.clone();

                    println!("{:?}", user_info_hashmap);

                    Ok(Response::new(AuthenicationAnswerResponse { session_id }))
                } else {
                    Err(Status::new(
                        Code::NotFound,
                        format!("User {} not found", user_id),
                    ))
                }
            } else {
                Err(Status::new(
                    Code::Unauthenticated,
                    format!("Challenge not solved correctly"),
                ))
            }
        } else {
            Err(Status::new(
                Code::NotFound,
                format!("Auth ID {} not found", auth_id),
            ))
        }
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
