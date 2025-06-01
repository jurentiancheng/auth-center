pub mod ctl;
pub mod entities;
pub mod mapper;
pub mod pojo;
pub mod svc;
pub mod util;

use std::{error::Error, sync::Arc, time::Duration};

use axum::{ http::StatusCode, routing::{get, post, put}, Json, Router
};

use ctl::user_ctl::{ UserCtl };
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use util::result_struct::RespResult;

pub type ResultJson<T> = Result<Json<RespResult<T>>, (StatusCode, Json<RespResult<String>>)>;


#[derive(Clone)]
pub struct AppState {
    mysql_pool: DatabaseConnection
}

async fn init_status() -> Result<Arc<AppState>, Box<dyn Error>> {
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



pub async fn build_app() -> Result<Router, Box<dyn Error>> {
    
    let app = Router::new()
        .route("/", get(UserCtl::root))
        .route(
            "/user",
            post(UserCtl::save)
                .put(UserCtl::update_by_id)
                .delete(UserCtl::remove_by_ids)
        )
        .route("/user/delByIds", put(UserCtl::delete_by_ids))
        .route("/user/list", get(UserCtl::list))
        .route("/user/page", get(UserCtl::page))
        .route("/user/:id", get(UserCtl::get_by_id))
        //.layer(middleware::from_extractor())
        .with_state(init_status().await?);
    Ok(app)
}


