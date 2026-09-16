//! 鉴权中间件：校验 `Authorization: Bearer <jwt>`，通过后把 `CurrentUser` 注入请求扩展。
//!
//! 只负责「是否已登录」。资源级权限（RBAC 权限码）不在这一层 —— `user` 与 `user_code`
//! 之间目前没有可用的关联约定，授权链路还建不起来（详见 readiness 评估）。

use std::sync::Arc;

use axum::extract::{Request, State};
use axum::http::header::AUTHORIZATION;
use axum::http::HeaderMap;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

use crate::auth::extractor::CurrentUser;
use crate::util::exception::unauthorized_err;
use crate::AppState;

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Response {
    let Some(token) = bearer_token(req.headers()) else {
        return unauthorized_err("缺少 Authorization: Bearer 令牌").into_response();
    };

    match state.jwt.verify(token) {
        Ok(claims) => {
            req.extensions_mut().insert(CurrentUser {
                user_id: claims.sub,
                user_name: claims.user_name,
                is_admin: claims.is_admin,
            });
            next.run(req).await
        }
        // 具体失败原因（过期 / 签名不符 / 格式错）只进日志，不回给客户端
        Err(err) => {
            tracing::debug!("令牌校验失败：{err}");
            unauthorized_err("令牌无效或已过期").into_response()
        }
    }
}

/// 从 `Authorization: Bearer xxx` 取出令牌，scheme 大小写不敏感。
fn bearer_token(headers: &HeaderMap) -> Option<&str> {
    let (scheme, token) = headers.get(AUTHORIZATION)?.to_str().ok()?.split_once(' ')?;
    if scheme.eq_ignore_ascii_case("bearer") && !token.trim().is_empty() {
        Some(token.trim())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn with_authorization(value: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, value.parse().unwrap());
        headers
    }

    #[test]
    fn parses_bearer_token() {
        assert_eq!(
            bearer_token(&with_authorization("Bearer a.b.c")),
            Some("a.b.c")
        );
        // scheme 大小写不敏感（RFC 7235）
        assert_eq!(bearer_token(&with_authorization("bearer a.b.c")), Some("a.b.c"));
        assert_eq!(bearer_token(&with_authorization("BEARER a.b.c")), Some("a.b.c"));
    }

    #[test]
    fn rejects_missing_or_malformed_authorization() {
        assert_eq!(bearer_token(&HeaderMap::new()), None, "没带头");
        assert_eq!(bearer_token(&with_authorization("a.b.c")), None, "没有 scheme");
        assert_eq!(bearer_token(&with_authorization("Basic a.b.c")), None, "换了 scheme");
        assert_eq!(bearer_token(&with_authorization("Bearer    ")), None, "空令牌");
    }
}
