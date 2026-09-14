use chrono::NaiveDateTime;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

use crate::util::date_format;

#[derive(FromQueryResult, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationVo {
    pub id: Option<i64>,
    pub code: Option<String>,
    pub parent_code: Option<String>,
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub contacts: Option<String>,
    pub cellphone: Option<String>,
    pub email: Option<String>,
    pub uscc: Option<String>,
    pub business_license: Option<String>,
    pub id_card_front: Option<String>,
    pub id_card_back: Option<String>,
    pub invalid_date: Option<chrono::NaiveDate>,
    pub status: Option<String>,
    pub extra: Option<serde_json::Value>,
    pub remark: Option<String>,
    pub path: Option<String>,
    pub is_del: Option<i8>,
    #[serde(with = "date_format")]
    pub create_time: Option<chrono::DateTime<chrono::Local>>,
    #[serde(with = "date_format")]
    pub update_time: Option<chrono::DateTime<chrono::Local>>,
    pub create_by: Option<i64>,
    pub update_by: Option<i64>,
    pub rec_sign: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationCondition {
    pub id: Option<i64>,
    pub ids: Option<Vec<i64>>,
    pub code: Option<String>,
    pub parent_code: Option<String>,
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub contacts: Option<String>,
    pub cellphone: Option<String>,
    pub email: Option<String>,
    pub uscc: Option<String>,
    pub status: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub page: Option<u64>,
    pub size: Option<u64>,
}

crate::impl_pageable!(OrganizationCondition);

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec_ids: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cellphone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uscc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_card_front: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_card_back: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<NaiveDateTime>,
}

crate::impl_into_json_value!(OrganizationDto);
