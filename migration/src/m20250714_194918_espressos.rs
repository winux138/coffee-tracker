use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "espressos",
            &[
            
            ("id", ColType::PkAuto),
            
            ("machine", ColType::String),
            ("dose_in", ColType::Double),
            ("dose_out", ColType::Float),
            ("temperature", ColType::IntegerNull),
            ("comment", ColType::StringNull),
            ("basket", ColType::String),
            ],
            &[
            ("beans", ""),
            ]
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "espressos").await
    }
}
