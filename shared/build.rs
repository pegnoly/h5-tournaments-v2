fn main() {
    tonic_prost_build::configure()
        .enum_attribute("HeroType", "#[derive(sea_orm::EnumIter, sea_orm::DeriveActiveEnum)]")
        .enum_attribute("HeroType", "#[sea_orm(rs_type = \"i32\", db_type = \"Integer\")]")
        .enum_attribute("UserStatus", "#[derive(sea_orm::EnumIter, sea_orm::DeriveActiveEnum)]")
        .enum_attribute("UserStatus", "#[sea_orm(rs_type = \"i32\", db_type = \"Integer\")]")
        .enum_attribute("CastleType", "#[derive(sea_orm::EnumIter, sea_orm::DeriveActiveEnum)]")
        .enum_attribute("CastleType", "#[sea_orm(rs_type = \"i32\", db_type = \"Integer\")]")
        .compile_protos(&["../proto/Auth.proto", "../proto/HeroType.proto", "../proto/Common.proto"], &["../proto/"]).unwrap();
}