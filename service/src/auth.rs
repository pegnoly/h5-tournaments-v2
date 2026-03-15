use tonic::{IntoRequest, Request, Response, Status};
use shared::auth_service_server::{AuthService, AuthServiceServer};
use shared::{RegistrationRequest, RegistrationResponse, UpdateLobbyNicknameRequest, UpdateLobbyNicknameResponse};

#[derive(Debug, Default)]
pub struct AuthServiceImpl {
    db: sea_orm::DatabaseConnection
}

impl AuthServiceImpl {
    pub fn new(db: sea_orm::DatabaseConnection) -> Self {
        Self { db }
    }
}

#[tonic::async_trait]
impl AuthService for AuthServiceImpl {
    async fn register(&self, request: Request<RegistrationRequest>) -> Result<Response<RegistrationResponse>, Status> {
        todo!()
    }

    async fn update_lobby_nickname(&self, request: Request<UpdateLobbyNicknameRequest>) -> Result<Response<UpdateLobbyNicknameResponse>, Status> {
        todo!()
    }
}