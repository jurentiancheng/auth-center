pub mod ctl;
pub mod entities;
pub mod mapper;
pub mod pojo;
pub mod route;
pub mod svc;
pub mod util;

use dotenvy::dotenv;
use std::{error::Error, sync::Arc, time::Duration};

use axum::{http::StatusCode, Json};

use sea_orm::{ConnectOptions, Database};
use util::result_struct::RespResult;

use crate::mapper::Mappers;
use crate::svc::Svcs;

pub type ResultJson<T> = Result<Json<RespResult<T>>, (StatusCode, Json<RespResult<String>>)>;

#[derive(Clone)]
pub struct AppState {
    svcs: Svcs,
}
impl AppState {
    pub fn new(svcs: Svcs) -> Self {
        Self { svcs }
    }
}

pub async fn init_status() -> Result<Arc<AppState>, Box<dyn Error>> {
    dotenv().ok();
    let db_uri = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_s| "mysql://root:root@127.0.0.1:18306/auth_center".to_string());
    let mut opt = ConnectOptions::new(db_uri);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(20))
        .sqlx_logging(false);
    let db: sea_orm::DatabaseConnection = Database::connect(opt).await?;

    Ok(Arc::new(AppState::new(Svcs::new(&Mappers::new(db)))))
}
