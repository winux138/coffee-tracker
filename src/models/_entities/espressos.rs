//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.11

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "espressos")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i32,
    pub machine: String,
    #[sea_orm(column_type = "Double")]
    pub dose_in: f64,
    #[sea_orm(column_type = "Float")]
    pub dose_out: f32,
    pub temperature: Option<i32>,
    pub comment: Option<String>,
    pub basket: String,
    pub bean_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::beans::Entity",
        from = "Column::BeanId",
        to = "super::beans::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Beans,
}

impl Related<super::beans::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Beans.def()
    }
}
