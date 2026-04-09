use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait};
use crate::error::ServiceError;
pub trait DBClient {
    type Entity: EntityTrait;
    type ActiveModel: ActiveModelTrait<Entity = Self::Entity>
        + Send
        + Sync
        + sea_orm::ActiveModelBehavior
        + IntoActiveModel<Self::ActiveModel>;
    async fn all(&self, database_connection: &DatabaseConnection) -> Result<Vec<<Self::Entity as EntityTrait>::Model>, ServiceError> {
        Self::Entity::find().all(database_connection).await.map_err(Into::into)
    }

    async fn insert(&self, database_connection: &DatabaseConnection, active_model: Self::ActiveModel) -> Result<<Self::Entity as EntityTrait>::Model, ServiceError> 
        where <<Self as DBClient>::Entity as EntityTrait>::Model: IntoActiveModel<<Self as DBClient>::ActiveModel> 
    {
        let x = Self::Entity::insert(active_model).exec_with_returning(database_connection).await?;
        Ok(x)
    }
}