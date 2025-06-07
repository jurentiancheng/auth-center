pub mod ctl;
pub mod entities;
pub mod mapper;
pub mod pojo;
pub mod svc;
pub mod util;
pub mod route;

use std::{error::Error, sync::Arc, time::Duration};

use axum::{ http::StatusCode, Json};

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use util::result_struct::RespResult;

pub type ResultJson<T> = Result<Json<RespResult<T>>, (StatusCode, Json<RespResult<String>>)>;


#[derive(Clone)]
pub struct AppState {
    mysql_pool: DatabaseConnection
}

pub async fn init_status() -> Result<Arc<AppState>, Box<dyn Error>> {
    let db_uri = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_s| "mysql://root:root@127.0.0.1:18306/auth_center".to_string());
    let mut opt = ConnectOptions::new(db_uri);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(20))
        .sqlx_logging(false);
    let db: sea_orm::DatabaseConnection = Database::connect(opt).await?;

    Ok(Arc::new(AppState { mysql_pool: db}))
}





