use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_dir = out_dir.ancestors()
        .find(|path| path.ends_with("h5-tournaments-v2"))
        .unwrap();
    tonic_prost_build::configure()
        .enum_attribute("HeroType", "#[derive(sea_orm::EnumIter, sea_orm::DeriveActiveEnum)]")
        .enum_attribute("HeroType", "#[sea_orm(rs_type = \"i32\", db_type = \"Integer\")]")
        .enum_attribute("UserStatus", "#[derive(sea_orm::EnumIter, sea_orm::DeriveActiveEnum)]")
        .enum_attribute("UserStatus", "#[sea_orm(rs_type = \"i32\", db_type = \"Integer\")]")
        .enum_attribute("CastleType", "#[derive(sea_orm::EnumIter, sea_orm::DeriveActiveEnum)]")
        .enum_attribute("CastleType", "#[sea_orm(rs_type = \"i32\", db_type = \"Integer\")]")
        .compile_protos(&[
            out_dir.join("proto/Auth.proto"),
            out_dir.join("proto/HeroType.proto"),
            out_dir.join("proto/Common.proto")
        ], &[out_dir.join("proto/")]).unwrap();
}