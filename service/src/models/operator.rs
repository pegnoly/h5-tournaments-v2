use sea_orm::prelude::*;
use shared_gen::hero_type_enum::HeroType;

#[sea_orm::model]
#[derive(Debug, Clone, Eq, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "operators")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    pub server_id: i64,
    pub generated_channel_id: i64,
    pub hero_type: HeroType
}

impl ActiveModelBehavior for ActiveModel {}