use chrono::NaiveDateTime;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::util::paged_struct::Pageable;
use crate::util::{common_func, IntoJsonValue};
use crate::util::date_format;

#[derive(FromQueryResult, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserWechatInfoVo {
    pub id: Option<i64>,
    pub union_id: Option<String>,
    pub wechat_open_id: Option<String>,
    pub mini_open_id: Option<String>,
    pub nickname: Option<String>,
    pub language: Option<String>,
    pub subscribe: Option<i8>,
    pub head_img_url: Option<String>,
    pub subscribe_time: Option<i64>,
    pub remark: Option<String>,
    pub group_id: Option<i32>,
    pub tag_ids: Option<serde_json::Value>,
    pub subscribe_scene: Option<String>,
    pub qr_scene: Option<String>,
    pub qr_scene_str: Option<String>,
    pub app_id: Option<String>,
    pub app_type: Option<String>,
    pub is_del: Option<i8>,
    #[serde(with = "date_format")]
    pub create_time: Option<chrono::DateTime<chrono::Local>>,
    #[serde(with = "date_format")]
    pub update_time: Option<chrono::DateTime<chrono::Local>>,
    pub create_by: Option<i64>,
    pub update_by: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserWechatInfoCondition {
    pub id: Option<i64>,
    pub ids: Option<Vec<i64>>,
    pub union_id: Option<String>,
    pub wechat_open_id: Option<String>,
    pub mini_open_id: Option<String>,
    pub nickname: Option<String>,
    pub subscribe_time: Option<i64>,
    pub group_id: Option<i32>,
    pub app_id: Option<String>,
    pub app_type: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub page: Option<u64>,
    pub size: Option<u64>,
}

impl Pageable for UserWechatInfoCondition {
    fn get_page(&self) -> Option<u64> {
        self.page.or(Some(1))
    }
    fn get_size(&self) -> Option<u64> {
        self.size.or(Some(20))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserWechatInfoDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec_ids: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_open_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mini_open_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_img_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_scene: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qr_scene: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qr_scene_str: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<NaiveDateTime>,
}

impl IntoJsonValue for UserWechatInfoDto {
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