#[macro_use]
pub mod macros;
pub mod auth;
pub mod config;
pub mod ctl;
pub mod entities;
pub mod hooks;
pub mod mapper;
pub mod pojo;
pub mod route;
pub mod svc;
pub mod util;

use std::{error::Error, sync::Arc};

use axum::{http::StatusCode, Json};

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use util::result_struct::RespResult;

use crate::auth::jwt::JwtService;
use crate::config::AppConfig;
use crate::mapper::Mappers;

pub type ResultJson<T> = Result<Json<RespResult<T>>, (StatusCode, Json<RespResult<String>>)>;

/// 应用状态：进程内共享依赖的装配结果。
///
/// 依赖统一从这里注入 —— `mappers` 是各资源的 mapper，`jwt` 是令牌服务。
/// 不再有 `OnceCell` 静态单例（此前 `get_instance` 是「首个 state 获胜」，
/// 既换不掉依赖也写不了测试）。
pub struct AppState {
    pub config: AppConfig,
    pub mappers: Mappers,
    pub jwt: Arc<JwtService>,
    pool: DatabaseConnection,
}

impl AppState {
    pub fn new(config: AppConfig, pool: DatabaseConnection) -> Self {
        Self {
            mappers: Mappers::new(pool.clone()),
            jwt: Arc::new(JwtService::new(&config.jwt_secret, config.jwt_expire_seconds)),
            config,
            pool,
        }
    }

    /// 连接池。字段保持私有，避免各处绕过 `Mappers` 直接拼 SQL。
    pub fn pool(&self) -> &DatabaseConnection {
        &self.pool
    }
}

pub async fn init_status() -> Result<Arc<AppState>, Box<dyn Error>> {
    let config = AppConfig::from_env()?;

    let mut opt = ConnectOptions::new(config.database_url.clone());
    opt.max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .connect_timeout(config.connect_timeout)
        .sqlx_logging(false);

    let db = Database::connect(opt).await?;
    Ok(Arc::new(AppState::new(config, db)))
}
