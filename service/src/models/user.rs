use chrono::NaiveDateTime;
use sea_orm::prelude::*;
use shared_gen::auth::UserStatus;
use crate::db::DBClient;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub discord_id: i64,
    pub discord_nickname: String,
    pub lobby_nickname: String,
    pub created_at: NaiveDateTime,
    pub status: UserStatus,
    pub is_banned: bool,
    pub ban_reason: Option<String>,
    pub ban_expires_at: Option<NaiveDateTime>
}
impl ActiveModelBehavior for ActiveModel {}

pub struct UserDBClient;

impl DBClient for UserDBClient {
    type Entity = Entity;
    type ActiveModel = ActiveModel;
}