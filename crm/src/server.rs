use anyhow::Result;
use crm::pb::{
    CreateUserRequest, GetUserRequest, User,
    user_service_server::{UserService, UserServiceServer},
};
use tonic::{Request, Response, Status, async_trait, transport::Server};

#[derive(Default)]
pub struct UserServer {}

#[async_trait]
impl UserService for UserServer {
    async fn get_user(&self, request: Request<GetUserRequest>) -> Result<Response<User>, Status> {
        let input = request.into_inner();
        println!("Get user request: {:?}", input);

        Ok(Response::new(User::default()))
    }
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<User>, Status> {
        let input = request.into_inner();
        println!("Create user request: {:?}", input);
        let user = User::new(1, &input.name, &input.email);
        Ok(Response::new(user))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "[::1]:50051";
    let svc = UserServer::default();

    Server::builder()
        .add_service(UserServiceServer::new(svc))
        .serve(addr.parse().unwrap())
        .await?;

    Ok(())
}
