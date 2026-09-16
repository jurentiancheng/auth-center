//! 应用配置：所有环境变量的唯一读取点。
//!
//! 之前 `DATABASE_URL` 之外的一切（端口、连接池、密钥）都散在代码里硬编码，
//! 这里统一收口并在启动时校验，缺关键配置直接失败而不是带着默认值跑起来。

use std::error::Error;
use std::fmt;
use std::time::Duration;

use dotenvy::dotenv;

const DEFAULT_DATABASE_URL: &str = "mysql://root:root@127.0.0.1:18306/auth_center";
const DEFAULT_LISTEN_ADDR: &str = "0.0.0.0:18080";
const DEFAULT_JWT_EXPIRE_SECONDS: i64 = 7200;
const DEFAULT_MAX_CONNECTIONS: u32 = 100;
const DEFAULT_MIN_CONNECTIONS: u32 = 5;
const DEFAULT_CONNECT_TIMEOUT_SECS: u64 = 20;

/// HS256 的密钥长度下限（字节）。短于 32 字节的密钥可被暴力枚举，
/// 与其在运行时被攻破，不如启动就拒绝。
const MIN_JWT_SECRET_LEN: usize = 32;

#[derive(Clone)]
pub struct AppConfig {
    pub database_url: String,
    /// JWT 签名密钥。无默认值，缺失即启动失败。
    pub jwt_secret: String,
    /// 令牌有效期（秒）
    pub jwt_expire_seconds: i64,
    pub listen_addr: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: Duration,
}

impl AppConfig {
    /// 读取 `.env` 与环境变量。`.env` 不存在不报错（生产一般直接注入环境变量）。
    pub fn from_env() -> Result<Self, Box<dyn Error>> {
        dotenv().ok();

        let jwt_secret = required("JWT_SECRET")?;
        if jwt_secret.len() < MIN_JWT_SECRET_LEN {
            return Err(format!(
                "JWT_SECRET 太短（{} 字节），HS256 密钥至少需要 {} 字节",
                jwt_secret.len(),
                MIN_JWT_SECRET_LEN
            )
            .into());
        }

        Ok(Self {
            database_url: optional("DATABASE_URL", DEFAULT_DATABASE_URL),
            jwt_secret,
            jwt_expire_seconds: parse_optional("JWT_EXPIRE_SECONDS", DEFAULT_JWT_EXPIRE_SECONDS)?,
            listen_addr: optional("LISTEN_ADDR", DEFAULT_LISTEN_ADDR),
            max_connections: parse_optional("DB_MAX_CONNECTIONS", DEFAULT_MAX_CONNECTIONS)?,
            min_connections: parse_optional("DB_MIN_CONNECTIONS", DEFAULT_MIN_CONNECTIONS)?,
            connect_timeout: Duration::from_secs(parse_optional(
                "DB_CONNECT_TIMEOUT_SECS",
                DEFAULT_CONNECT_TIMEOUT_SECS,
            )?),
        })
    }
}

/// 手写 Debug：`jwt_secret` 不能进日志。
impl fmt::Debug for AppConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppConfig")
            .field("database_url", &self.database_url)
            .field("jwt_secret", &"[redacted]")
            .field("jwt_expire_seconds", &self.jwt_expire_seconds)
            .field("listen_addr", &self.listen_addr)
            .field("max_connections", &self.max_connections)
            .field("min_connections", &self.min_connections)
            .field("connect_timeout", &self.connect_timeout)
            .finish()
    }
}

fn optional(key: &str, default: &str) -> String {
    match std::env::var(key) {
        Ok(v) if !v.trim().is_empty() => v,
        _ => default.to_string(),
    }
}

fn required(key: &str) -> Result<String, Box<dyn Error>> {
    match std::env::var(key) {
        Ok(v) if !v.trim().is_empty() => Ok(v),
        _ => Err(format!(
            "缺少必需的环境变量 {key}：认证服务不会为 JWT 密钥提供默认值。\n\
             \x20 本地开发：在 .env 里加上 {key}=<随机串>（变量清单见 .env.example）\n\
             \x20 生成密钥：openssl rand -base64 48"
        )
        .into()),
    }
}

fn parse_optional<T>(key: &str, default: T) -> Result<T, Box<dyn Error>>
where
    T: std::str::FromStr,
    T::Err: Error + 'static,
{
    match std::env::var(key) {
        Ok(v) if !v.trim().is_empty() => Ok(v.trim().parse::<T>()?),
        _ => Ok(default),
    }
}
