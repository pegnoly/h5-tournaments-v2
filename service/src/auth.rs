use std::sync::Arc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, Set};
use tonic::{Request, Response, Status};
use shared::auth_service_server::{AuthService, AuthServiceServer};
use shared::{RegistrationRequest, RegistrationResponse, UpdateLobbyNicknameRequest, UpdateLobbyNicknameResponse};
use crate::db::DBClient;
use crate::models::user;
use crate::models::user::UserDBClient;
pub struct AuthServiceImpl<C: DBClient<Entity = user::Entity>> {
    db: DatabaseConnection,
    user_db_client: C
}

impl AuthServiceImpl<UserDBClient> {
    pub fn new(db: DatabaseConnection) -> Self {
        AuthServiceImpl { user_db_client: UserDBClient, db }
    }
}

#[tonic::async_trait]
impl AuthService for AuthServiceImpl<UserDBClient> {
    async fn register(&self, request: Request<RegistrationRequest>) -> Result<Response<RegistrationResponse>, Status> {
        let request = request.into_inner();
        let model = user::ActiveModel {
            discord_id: Set(request.discord_id),
            discord_nickname: Set(request.discord_nickname),
            lobby_nickname: Set(request.lobby_nickname),
            ..Default::default()
        };

        match self.user_db_client.insert(&self.db, model).await {
            Ok(result) => Ok(Response::new(RegistrationResponse {
                success: true,
                message: "User registered successfully".to_string(),
                user_id: Some(result.id)
            })),
            Err(e) => Err(Status::internal(e.to_string()))
        }
    }

    async fn update_lobby_nickname(&self, request: Request<UpdateLobbyNicknameRequest>) -> Result<Response<UpdateLobbyNicknameResponse>, Status> {
        let request = request.into_inner();
        match user::Entity::find_by_id(request.user_id).one(&self.db).await {
            Ok(Some(user)) => {
                let mut model_to_update = user.into_active_model();
                model_to_update.lobby_nickname = Set(request.new_nickname);
                match model_to_update.update(&self.db).await {
                    Ok(_) => Ok(Response::new(UpdateLobbyNicknameResponse { success: true, message: "Nickname updated".to_string() })),
                    Err(e) => Err(Status::internal(e.to_string()))
                }
            },
            Ok(None) => {
                return Err(Status::not_found(format!("User {} not found", request.user_id)))
            }
            Err(e) => return Err(Status::aborted(e.to_string()))
        }
    }
}