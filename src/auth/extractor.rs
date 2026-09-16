//! `CurrentUser`：鉴权中间件注入的当前登录身份。
//!
//! 业务 handler 直接把它当提取器写进参数即可拿到当前用户，不必重复解析令牌：
//!
//! ```ignore
//! pub async fn whoami(user: CurrentUser) -> ResultJson<CurrentUser> { ... }
//! ```
//!
//! 注意：axum 0.7 的 `FromRequestParts` 是 `#[async_trait]` 的，而 derive 宏在未启用的
//! `macros` feature 后面，所以这里手写实现。

use std::sync::Arc;

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;

use crate::util::exception::unauthorized_err;
use crate::util::result_struct::RespResult;
use crate::AppState;

/// 当前登录用户。由 [`crate::auth::middleware::auth_middleware`] 校验令牌后写入请求扩展。
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentUser {
    pub user_id: i64,
    pub user_name: String,
    pub is_admin: bool,
}

#[async_trait::async_trait]
impl FromRequestParts<Arc<AppState>> for CurrentUser {
    type Rejection = (StatusCode, Json<RespResult<String>>);

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        // 中间件正常情况下已经注入。取不到说明这个路由没有挂在鉴权保护层后面 ——
        // 宁可 401 也不要静默放行（否则漏挂一层中间件就等于接口裸奔）。
        parts
            .extensions
            .get::<CurrentUser>()
            .cloned()
            .ok_or_else(|| unauthorized_err("缺少登录态，请先登录"))
    }
}
