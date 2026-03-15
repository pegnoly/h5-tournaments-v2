use shared::auth_service_server::AuthServiceServer;
use crate::auth::AuthServiceImpl;
use crate::error::ServiceError;
mod auth;
mod models;
mod error;

#[tokio::main]
async fn main() -> Result<(), ServiceError> {
    dotenv::dotenv().ok();
    let addr = format!("[::1]:{}", std::env::var("PORT")?).parse().unwrap();
    let db_connection = sea_orm::Database::connect(std::env::var("DB_URL")?).await?;
    let auth_service = AuthServiceImpl::new(db_connection);

    tonic::transport::Server::builder()
        .add_service(AuthServiceServer::new(auth_service))
        .serve(addr)
        .await?;
    Ok(())
}
