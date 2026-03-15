use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error(transparent)]
    DB(#[from] sea_orm::error::DbErr),
    #[error(transparent)]
    Transport(#[from] tonic::transport::Error)
}