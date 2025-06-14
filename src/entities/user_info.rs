//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.12

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user_info")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_code: Option<String>,
    pub username: Option<String>,
    pub login_status: Option<String>,
    pub english_name: Option<String>,
    pub real_name: String,
    pub nick_name: Option<String>,
    pub cellphone: Option<String>,
    pub gender: Option<String>,
    pub portrait: Option<String>,
    pub user_type: String,
    pub birthday: Option<Date>,
    pub password: Option<String>,
    pub id_card_type: Option<String>,
    pub id_card_no: Option<String>,
    pub email: Option<String>,
    pub qq: Option<String>,
    pub wx_union_id: Option<String>,
    pub wx_open_id: Option<String>,
    pub wx_mini_open_id: Option<String>,
    pub address: Option<String>,
    pub parent_code: Option<String>,
    pub path: Option<String>,
    pub effective_date: Option<Date>,
    pub invalid_date: Option<Date>,
    pub status: Option<String>,
    pub org_code: Option<String>,
    pub department_code: Option<String>,
    pub position_code: Option<String>,
    pub extra: Option<Json>,
    pub remark: Option<String>,
    #[sea_orm(default_value = 0, nullable)]
    pub is_del: Option<i8>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
    #[sea_orm(default_value = 0, nullable)]
    pub create_by: Option<i64>,
    #[sea_orm(default_value = 0, nullable)]
    pub update_by: Option<i64>,
    pub rec_sign: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
