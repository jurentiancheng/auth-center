//! 认证领域服务：登录、签发令牌。

use std::fmt;
use std::sync::Arc;

use sea_orm::DbErr;

use crate::auth::jwt::JwtService;
use crate::auth::mapper::AuthMapperTrait;
use crate::auth::password;
use crate::pojo::auth_pojo::{LoginDto, LoginVo};

/// 对外统一的登录失败文案。
///
/// 不区分「用户不存在」「密码错误」「账号被禁用」—— 否则登录接口就变成了用户名枚举器。
pub const LOGIN_FAILED_MSG: &str = "用户名或密码错误";

#[derive(Debug)]
pub enum AuthError {
    /// 凭证无效 —— 一律 401
    Unauthorized,
    /// 服务端问题（数据库、令牌签发失败） —— 500
    Internal(String),
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::Unauthorized => write!(f, "{LOGIN_FAILED_MSG}"),
            AuthError::Internal(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for AuthError {}

impl From<DbErr> for AuthError {
    fn from(err: DbErr) -> Self {
        AuthError::Internal(format!("数据库错误：{err}"))
    }
}

pub struct AuthSvc {
    mapper: Arc<dyn AuthMapperTrait>,
    jwt: Arc<JwtService>,
}

impl AuthSvc {
    pub fn new(mapper: Arc<dyn AuthMapperTrait>, jwt: Arc<JwtService>) -> Self {
        Self { mapper, jwt }
    }

    pub async fn login(&self, dto: LoginDto) -> Result<LoginVo, AuthError> {
        let user = self
            .mapper
            .find_by_user_name(&dto.user_name)
            .await?
            .ok_or(AuthError::Unauthorized)?;

        // `status` 在库里没有字典表，这里约定 0 = 启用（与 init.sql 的 DEFAULT 0 一致），
        // 非 0 视为不可登录。要改语义只改这一处。
        // `is_del` 已由 find_by_user_name 的查询条件过滤掉，不再重复判断。
        if user.status.unwrap_or(0) != 0 {
            return Err(AuthError::Unauthorized);
        }

        // 没设过密码的账号（password 为 NULL）不可登录
        let stored = user.password.as_deref().ok_or(AuthError::Unauthorized)?;
        if !password::verify_password(&dto.password, stored) {
            return Err(AuthError::Unauthorized);
        }

        let user_id = user
            .id
            .ok_or_else(|| AuthError::Internal("user.id 缺失".to_string()))?;
        let user_name = user.user_name.unwrap_or_default();
        let is_admin = user.is_admin.unwrap_or(0) != 0;

        let token = self
            .jwt
            .sign(user_id, &user_name, is_admin)
            .map_err(AuthError::Internal)?;

        // 登录已经成功，写登录时间失败不该把用户挡在门外，记日志即可。
        if let Err(err) = self.mapper.touch_last_login(user_id).await {
            tracing::warn!(user_id, "写入最后登录时间失败：{err}");
        }

        Ok(LoginVo {
            token,
            token_type: "Bearer".to_string(),
            expires_in: self.jwt.expire_seconds(),
            user_id,
            user_name,
            is_admin,
        })
    }
}
