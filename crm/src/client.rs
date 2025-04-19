use anyhow::Result;
use crm::pb::{CreateUserRequest, user_service_client::UserServiceClient};
use tonic::Request;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "http://[::1]:50051";
    let mut client = UserServiceClient::connect(addr).await?;

    let request = Request::new(CreateUserRequest {
        name: "John Doe".to_string(),
        email: "JohnDoe@gmail.com".to_string(),
    });

    let response = client.create_user(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

#[cfg(test)]
mod tests {}
