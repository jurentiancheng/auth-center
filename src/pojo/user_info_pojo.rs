use chrono::NaiveDateTime;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::util::paged_struct::Pageable;
use crate::util::{common_func, IntoJsonValue};
use crate::util::date_format;

#[derive(FromQueryResult, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfoVo {
    pub id: Option<i64>,
    pub user_code: Option<String>,
    pub username: Option<String>,
    pub login_status: Option<String>,
    pub english_name: Option<String>,
    pub real_name: Option<String>,
    pub nick_name: Option<String>,
    pub cellphone: Option<String>,
    pub gender: Option<String>,
    pub portrait: Option<String>,
    pub user_type: Option<String>,
    pub birthday: Option<chrono::NaiveDate>,
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
    pub effective_date: Option<chrono::NaiveDate>,
    pub invalid_date: Option<chrono::NaiveDate>,
    pub status: Option<String>,
    pub org_code: Option<String>,
    pub department_code: Option<String>,
    pub position_code: Option<String>,
    pub extra: Option<serde_json::Value>,
    pub remark: Option<String>,
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
pub struct UserInfoCondition {
    pub id: Option<i64>,
    pub ids: Option<Vec<i64>>,
    pub user_code: Option<String>,
    pub username: Option<String>,
    pub real_name: Option<String>,
    pub nick_name: Option<String>,
    pub gender: Option<String>,
    pub cellphone: Option<String>,
    pub id_card_type: Option<String>,
    pub id_card_no: Option<String>,
    pub wx_union_id: Option<String>,
    pub wx_open_id: Option<String>,
    pub wx_mini_open_id: Option<String>,
    pub qq: Option<String>, 
    pub parent_code: Option<String>,
    pub path: Option<String>,
    pub email: Option<String>,
    pub user_type: Option<String>,
    pub status: Option<String>,
    pub org_code: Option<String>,
    pub department_code: Option<String>,
    pub position_code: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub page: Option<u64>,
    pub size: Option<u64>,
}

impl Pageable for UserInfoCondition {
    fn get_page(&self) -> Option<u64> {
        self.page.or(Some(1))
    }
    fn get_size(&self) -> Option<u64> {
        self.size.or(Some(20))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserInfoDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec_ids: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub english_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cellphone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portrait: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_card_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_card_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qq: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wx_union_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wx_open_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wx_mini_open_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<NaiveDateTime>,
}

impl IntoJsonValue for UserInfoDto {
    fn into_json_with_snake_key(&self) -> serde_json::Value {
        let mut json_object = serde_json::Map::new();
        let json_value = json!(self);
        if json_value.is_object() {
            let obj_map = json_value.as_object().unwrap();
            for (k, v) in obj_map {
                json_object.insert(
                    common_func::camel_case_to_under_score(k.clone().as_str()),
                    v.clone(),
                );
            }
        }
        Value::Object(json_object)
    }
} 