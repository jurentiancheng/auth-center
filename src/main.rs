use std::error::Error;


use auth_center::build_app;
use tracing::{info, instrument};

#[tokio::main]
#[instrument]
async fn main() -> Result<(), Box<dyn Error>> {
    // 只有注册 subscriber 后， 才能在控制台上看到日志输出
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .with_test_writer()
    .init();

    let app = build_app().await.unwrap();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:18080").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
