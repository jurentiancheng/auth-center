//! 认证接口。
//!
//! 手写 handler，不走 `impl_controller!` —— 登录不是 CRUD，
//! 也顺带证明宏生成的 handler 与手写 handler 能挂在同一个 Router 上。

use std::sync::Arc;

use axum::{extract::State, Json};

use crate::auth::extractor::CurrentUser;
use crate::auth::svc::{AuthError, AuthSvc, LOGIN_FAILED_MSG};
use crate::pojo::auth_pojo::{LoginDto, LoginVo};
use crate::util::exception::{internal_err, unauthorized_err};
use crate::util::result_struct::RespResult;
use crate::{AppState, ResultJson};

pub struct AuthCtl;

impl AuthCtl {
    /// `POST /auth/login` —— 白名单接口，无需令牌。
    pub async fn login(
        State(state): State<Arc<AppState>>,
        Json(dto): Json<LoginDto>,
    ) -> ResultJson<LoginVo> {
        let svc = AuthSvc::new(state.mappers.auth.clone(), state.jwt.clone());

        match svc.login(dto).await {
            Ok(vo) => Ok(Json(RespResult::ok(vo))),
            // 凭证问题一律 401，文案与外层统一
            Err(AuthError::Unauthorized) => Err(unauthorized_err(LOGIN_FAILED_MSG)),
            Err(err) => Err(internal_err(err)),
        }
    }

    /// `GET /auth/me` —— 需令牌。回显当前登录身份，
    /// 客户端也可以拿它当作「令牌是否还有效」的探针。
    pub async fn me(user: CurrentUser) -> ResultJson<CurrentUser> {
        Ok(Json(RespResult::ok(user)))
    }
}
