use serde_json::Value;

pub trait IntoJsonValue {
    fn into_json_with_snake_key(&self) -> Value;
}

pub mod common_func {

    /**
     * CamelCase to UnderScore
     */
    pub fn camel_case_to_under_score(camel_case: &str) -> String {
        camel_case
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if c.is_uppercase() && i > 0 {
                    format!("_{}", c.to_ascii_lowercase())
                } else {
                    c.to_string()
                }
            })
            .collect::<String>()
    }
}

pub mod exception {

    use super::result_struct::RespResult;
    use axum::{http::StatusCode, Json};

    pub fn internal_err<E>(err: E) -> (StatusCode, Json<RespResult<String>>)
    where
        E: std::error::Error,
    {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(RespResult::sys_error(err.to_string())),
        )
    }

    // anyhow implament
    pub fn internal_anyhow_err(err: anyhow::Error) -> (StatusCode, Json<RespResult<String>>)
    {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(RespResult::sys_error(err.to_string())),
        )
    }

}

pub mod result_struct {
    use std::any::Any;

    use serde::Serialize;

    #[derive(Serialize, Debug, Default)]
    pub struct RespResult<T> {
        pub code: i32,
        pub data: T,
        pub message: String,
    }

    impl<T> RespResult<T>
    where
        T: Any,
    {
        pub fn ok(data: T) -> Self {
            Self {
                code: 0,
                message: "ok".to_string(),
                data: data,
            }
        }
        pub fn sys_error(data: T) -> Self {
            Self {
                code: 500,
                message: "系统错误！".to_string(),
                data: data,
            }
        }
    }
}

pub mod paged_struct {
    use serde::Serialize;

    #[derive(Debug, Serialize, Default)]
    pub struct PageInfo {
        page: u64,
        size: u64,
        total: u64,
    }

    impl PageInfo {
        pub fn from(page: u64, size: u64, total: u64) -> Self {
            Self { page, size, total }
        }
    }

    #[derive(Debug, Default, Serialize)]
    pub struct PageData<T> {
        pub page_info: PageInfo,
        pub page_data: Vec<T>,
    }

    impl<T> PageData<T>
    where
        T: Serialize,
    {
        pub fn new(page_info: PageInfo, page_data: Vec<T>) -> Self {
            Self {
                page_info,
                page_data,
            }
        }
    }

    pub trait Pageable {
        fn get_page(&self) -> Option<u64>;
        fn get_size(&self) -> Option<u64> {
            Some(20)
        }
        fn get_offset(&self) -> Option<u64> {
            let page = match self.get_page() {
                Some(page) => page,
                None => 1,
            };
            let size = match self.get_size() {
                Some(size) => size,
                None => 20,
            };
            return if page == 1 {
                Some(0)
            } else {
                Some((page - 1) * size)
            };
        }
    }
}

pub mod date_format {


    use sea_orm::prelude::DateTimeLocal;
    use serde::Serializer;
    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &Option<DateTimeLocal>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if date.is_none() {
            serializer.serialize_none()
        } else {
            let s = format!("{}", date.unwrap().format(FORMAT));
            serializer.serialize_str(&s)
        }
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    /*
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTimeLocal>, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let s: String = String::deserialize(deserializer)?;
            let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
            Ok(Some(dt))
        } else {
            Ok(Some(NaiveDateTime::MIN))
        }
        
    }
    */
}
