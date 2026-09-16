//! 鉴权闸门的端到端行为。
//!
//! 用 `DatabaseConnection::Disconnected` 装配 `AppState` —— 401 路径上中间件根本不会碰库，
//! 带合法令牌的那个用例走的也是不查库的假 handler，所以整份测试不需要数据库。
//!
//! 覆盖的是「漏挂中间件 = 接口裸奔」这类最该被守住的回归。

use std::sync::Arc;
use std::time::Duration;

use auth_center::auth::extractor::CurrentUser;
use auth_center::auth::jwt::JwtService;
use auth_center::auth::middleware::auth_middleware;
use auth_center::config::AppConfig;
use auth_center::util::result_struct::RespResult;
use auth_center::{AppState, ResultJson};
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::middleware;
use axum::routing::get;
use axum::{Json, Router};
use sea_orm::DatabaseConnection;
use tower::ServiceExt;

const SECRET: &str = "integration-test-secret-0123456789abcdef";

fn test_state() -> Arc<AppState> {
    let config = AppConfig {
        database_url: String::new(),
        jwt_secret: SECRET.to_string(),
        jwt_expire_seconds: 3600,
        listen_addr: "0.0.0.0:0".to_string(),
        max_connections: 1,
        min_connections: 0,
        connect_timeout: Duration::from_secs(1),
    };
    Arc::new(AppState::new(config, DatabaseConnection::Disconnected))
}

/// 受保护路由上的假 handler：把中间件注入的 `CurrentUser` 回显出来，
/// 以此证明「中间件确实把当前登录身份交给了业务代码」。
fn protected_app(state: Arc<AppState>) -> Router {
    async fn whoami(user: CurrentUser) -> ResultJson<CurrentUser> {
        Ok(Json(RespResult::ok(user)))
    }

    Router::new()
        .route("/protected", get(whoami))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(state)
}

async fn call(authorization: Option<&str>) -> (StatusCode, String) {
    let mut builder = Request::builder().uri("/protected");
    if let Some(value) = authorization {
        builder = builder.header("Authorization", value);
    }
    let request = builder.body(Body::empty()).unwrap();

    let response = protected_app(test_state()).oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();

    (status, String::from_utf8_lossy(&bytes).into_owned())
}

#[tokio::test]
async fn missing_authorization_is_rejected() {
    let (status, body) = call(None).await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    // HTTP 状态码与 body.code 必须一致，否则客户端不知道该信哪个
    assert!(body.contains("\"code\":401"), "响应体：{body}");
}

#[tokio::test]
async fn wrong_scheme_is_rejected() {
    let (status, _body) = call(Some("Basic YWxpY2U6cHc=")).await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn garbage_token_is_rejected() {
    let (status, _body) = call(Some("Bearer not-a-jwt")).await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn token_signed_with_another_secret_is_rejected() {
    let foreign = JwtService::new("some-other-secret-0123456789abcdefgh", 3600);
    let token = foreign.sign(7, "alice", true).unwrap();
    let header = format!("Bearer {token}");

    let (status, _body) = call(Some(header.as_str())).await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn valid_token_passes_and_current_user_reaches_the_handler() {
    let token = JwtService::new(SECRET, 3600)
        .sign(7, "alice", true)
        .unwrap();
    let header = format!("Bearer {token}");

    let (status, body) = call(Some(header.as_str())).await;

    assert_eq!(status, StatusCode::OK, "合法令牌应放行，响应体：{body}");
    assert!(body.contains("\"userId\":7"), "应注入当前用户，响应体：{body}");
    assert!(body.contains("\"userName\":\"alice\""), "响应体：{body}");
    assert!(body.contains("\"isAdmin\":true"), "响应体：{body}");
}
