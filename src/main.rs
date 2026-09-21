use std::error::Error;
use std::sync::Arc;

use auth_center::config::Config;
use auth_center::route::build_app_route;
use tracing::{info, instrument};
use tracing_subscriber::EnvFilter;

#[tokio::main]
#[instrument]
async fn main() -> Result<(), Box<dyn Error>> {
    // 先读配置：日志级别本身也来自配置，必须早于 subscriber 初始化。
    let config = Arc::new(Config::from_env()?);

    // 只有注册 subscriber 后， 才能在控制台上看到日志输出
    let filter = EnvFilter::try_new(&config.log.level).unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_test_writer()
        .init();

    let app = build_app_route(config.clone()).await?;
    let listener = tokio::net::TcpListener::bind(config.server.bind_addr()).await?;
    info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
