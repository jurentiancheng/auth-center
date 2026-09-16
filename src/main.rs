use std::error::Error;


use auth_center::route::build_app_route;
use tracing::{info, instrument};

#[tokio::main]
#[instrument]
async fn main() -> Result<(), Box<dyn Error>> {
    // 只有注册 subscriber 后， 才能在控制台上看到日志输出。
    // 级别由 RUST_LOG 控制（如 RUST_LOG=debug），缺省 info。
    // 原本这里写死了 DEBUG 且用了 with_test_writer()（那是给测试用的 writer）。
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let (app, state) = build_app_route().await?;

    let listener = tokio::net::TcpListener::bind(&state.config.listen_addr).await?;
    info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
