pub mod config;
pub mod ctl;
pub mod entities;
pub mod mapper;
pub mod pojo;
pub mod route;
pub mod svc;
pub mod util;

use std::{error::Error, sync::Arc};

use axum::{http::StatusCode, Json};

use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use util::result_struct::RespResult;

use crate::config::{Config, DatabaseConfig};
use crate::mapper::Mappers;
use crate::svc::Svcs;

pub type ResultJson<T> = Result<Json<RespResult<T>>, (StatusCode, Json<RespResult<String>>)>;

/// 全局应用状态，随 `Router` 一起交给 axum，控制器用
/// `State(state): State<Arc<AppState>>` 取用。
#[derive(Clone)]
pub struct AppState {
    /// 启动时装载一次的只读配置，见 [`crate::config`]。
    pub config: Arc<Config>,
    svcs: Svcs,
}

impl AppState {
    pub fn new(config: Arc<Config>, svcs: Svcs) -> Self {
        Self { config, svcs }
    }
}

/// 按配置建立数据库连接池。
async fn connect_database(cfg: &DatabaseConfig) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(cfg.url.clone());
    opt.max_connections(cfg.max_connections)
        .min_connections(cfg.min_connections)
        .connect_timeout(cfg.connect_timeout)
        .sqlx_logging(cfg.sqlx_logging);
    Database::connect(opt).await
}

/// 装配全局状态：建连接池 → 建 [`Mappers`] → 建 [`Svcs`] → 挂上 [`Config`]。
pub async fn init_status(config: Arc<Config>) -> Result<Arc<AppState>, Box<dyn Error>> {
    let db = connect_database(&config.database).await?;
    Ok(Arc::new(AppState::new(
        config,
        Svcs::new(&Mappers::new(db)),
    )))
}
