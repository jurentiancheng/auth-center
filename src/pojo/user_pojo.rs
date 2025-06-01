
use chrono::NaiveDateTime;
use sea_orm::{prelude::DateTimeLocal, FromQueryResult};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::util::paged_struct::Pageable;
use crate::util::{common_func, IntoJsonValue};
use crate::util::date_format;

#[derive(FromQueryResult, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserVo {
    pub id: Option<i64>,
    pub user_name: Option<String>,
    pub real_name: Option<String>,
    pub password: Option<String>,
    pub r#type: Option<i8>,
    pub status: Option<i8>,
    pub email: Option<String>,
    pub area_code: Option<String>,
    pub phone: Option<String>,
    pub remark: Option<String>,
    pub is_del: Option<i8>,
    pub head_pic: Option<String>,
    #[serde(with = "date_format")]
    pub create_time: Option<DateTimeLocal>,
    #[serde(with = "date_format")]
    pub update_time: Option<DateTimeLocal>,
    pub create_by: Option<i64>,
    pub update_by: Option<i64>,
    pub is_admin: Option<i8>,
    pub open_id: Option<String>,
    pub last_login_time: Option<DateTimeLocal>,
    pub wechat_open_id: Option<String>,
    pub wechat_union_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserCondition {
    pub id: Option<i64>,
    pub ids: Option<Vec<i64>>,
    pub user_name: Option<String>,
    pub real_name: Option<String>,
    pub r#type: Option<i8>,
    pub status: Option<i8>,
    pub email: Option<String>,
    pub area_code: Option<String>,
    pub phone: Option<String>,
    pub remark: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub open_id: Option<String>,
    pub wechat_open_id: Option<String>,
    pub wechat_union_id: Option<String>,
    pub page: Option<u64>,
    pub size: Option<u64>,
}

impl Pageable for UserCondition {
    fn get_page(&self) -> Option<u64> {
        return if let Some(page) = self.page {
            Some(page)
        } else {
            Some(1)
        };
    }
    fn get_size(&self) -> Option<u64> {
        return if let Some(size) = self.size {
            Some(size)
        } else {
            Some(20)
        };
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec_ids: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_name: Option<String>,
    #[serde(default = "default_resource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i8>,
    #[serde(default = "default_resource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    //#[serde(with = "date_format")]// if not set skip serialize
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub update_time: Option<NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_open_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_union_id: Option<String>,
}
// serde field default value
fn default_resource() -> Option<i8> {
    Some(0_i8)
}

impl IntoJsonValue for UserDto {
    /**
     * 转换 UserDto 对象为 serde_json::Value，对象Key转化为SnakeValue
     */
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
