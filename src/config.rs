//! 应用配置：统一从环境变量读取（本地开发由项目根目录的 `.env` 提供，见 `.env.example`）。
//!
//! 进程启动时由 [`Config::from_env`] 装载一次，之后以 `Arc<Config>` 的形式挂在
//! [`crate::AppState`] 上全局只读共享，运行期不再变化。
//!
//! 所有配置项都带默认值，本地不写 `.env` 也能跑起来；部署时用环境变量覆盖即可。

use std::env;
use std::fmt;
use std::time::Duration;

/// 环境变量已设置、但值无法解析成目标类型时返回的错误。
#[derive(Debug)]
pub struct ConfigError {
    key: String,
    value: String,
    reason: String,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "配置项 `{}` 的值 `{}` 非法：{}",
            self.key, self.value, self.reason
        )
    }
}

impl std::error::Error for ConfigError {}

const DEFAULT_SERVER_HOST: &str = "0.0.0.0";
const DEFAULT_SERVER_PORT: u16 = 18080;
const DEFAULT_DATABASE_URL: &str = "mysql://root:root@127.0.0.1:18306/auth_center";
const DEFAULT_DATABASE_MAX_CONNECTIONS: u32 = 100;
const DEFAULT_DATABASE_MIN_CONNECTIONS: u32 = 5;
const DEFAULT_DATABASE_CONNECT_TIMEOUT_SECS: u64 = 20;
const DEFAULT_LOG_LEVEL: &str = "debug";

/// 应用全部配置，按用途分组。
#[derive(Debug, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub log: LogConfig,
}

impl Config {
    /// 读取全部配置。会先尝试加载 `.env`（文件不存在时静默跳过，
    /// 已存在的进程环境变量优先级更高）。
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();
        Ok(Self {
            server: ServerConfig::from_env()?,
            database: DatabaseConfig::from_env()?,
            log: LogConfig::from_env()?,
        })
    }
}

/// HTTP 服务监听配置。
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// `SERVER_HOST`，默认 `0.0.0.0`
    pub host: String,
    /// `SERVER_PORT`，默认 `18080`
    pub port: u16,
}

impl ServerConfig {
    fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            host: env_str("SERVER_HOST", DEFAULT_SERVER_HOST)?,
            port: env_parse("SERVER_PORT", DEFAULT_SERVER_PORT)?,
        })
    }

    /// `host:port` 形式的监听地址，可直接交给 `TcpListener::bind`。
    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

/// 数据库连接池配置。
#[derive(Clone)]
pub struct DatabaseConfig {
    /// `DATABASE_URL`
    pub url: String,
    /// `DATABASE_MAX_CONNECTIONS`，默认 `100`
    pub max_connections: u32,
    /// `DATABASE_MIN_CONNECTIONS`，默认 `5`
    pub min_connections: u32,
    /// `DATABASE_CONNECT_TIMEOUT_SECS`，默认 `20`
    pub connect_timeout: Duration,
    /// `DATABASE_SQLX_LOGGING`，默认 `false`
    pub sqlx_logging: bool,
}

impl DatabaseConfig {
    fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            url: env_str("DATABASE_URL", DEFAULT_DATABASE_URL)?,
            max_connections: env_parse(
                "DATABASE_MAX_CONNECTIONS",
                DEFAULT_DATABASE_MAX_CONNECTIONS,
            )?,
            min_connections: env_parse(
                "DATABASE_MIN_CONNECTIONS",
                DEFAULT_DATABASE_MIN_CONNECTIONS,
            )?,
            connect_timeout: Duration::from_secs(env_parse(
                "DATABASE_CONNECT_TIMEOUT_SECS",
                DEFAULT_DATABASE_CONNECT_TIMEOUT_SECS,
            )?),
            sqlx_logging: env_bool("DATABASE_SQLX_LOGGING", false)?,
        })
    }
}

/// 手写 `Debug`：连接串里带着密码，直接打印会把凭据写进日志。
impl fmt::Debug for DatabaseConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DatabaseConfig")
            .field("url", &redact_url(&self.url))
            .field("max_connections", &self.max_connections)
            .field("min_connections", &self.min_connections)
            .field("connect_timeout", &self.connect_timeout)
            .field("sqlx_logging", &self.sqlx_logging)
            .finish()
    }
}

/// 日志配置。
#[derive(Debug, Clone)]
pub struct LogConfig {
    /// `LOG_LEVEL`，取值同 `RUST_LOG`（如 `debug`、`info,auth_center=trace`），默认 `debug`
    pub level: String,
}

impl LogConfig {
    fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            level: env_str("LOG_LEVEL", DEFAULT_LOG_LEVEL)?,
        })
    }
}

/// 读取环境变量；未设置、或值是空白串时返回 `None`（空白串视为「没配」，
/// 这样 `.env` 里写 `SERVER_PORT=` 也能回落到默认值）。
fn env_raw(key: &str) -> Result<Option<String>, ConfigError> {
    match env::var(key) {
        Ok(value) if !value.trim().is_empty() => Ok(Some(value)),
        Ok(_) => Ok(None),
        Err(env::VarError::NotPresent) => Ok(None),
        Err(env::VarError::NotUnicode(_)) => Err(ConfigError {
            key: key.to_string(),
            value: "<非 UTF-8>".to_string(),
            reason: "环境变量的值不是合法的 UTF-8".to_string(),
        }),
    }
}

fn env_str(key: &str, default: &str) -> Result<String, ConfigError> {
    Ok(env_raw(key)?.unwrap_or_else(|| default.to_string()))
}

fn env_parse<T>(key: &str, default: T) -> Result<T, ConfigError>
where
    T: std::str::FromStr,
    T::Err: fmt::Display,
{
    match env_raw(key)? {
        None => Ok(default),
        Some(raw) => raw.trim().parse::<T>().map_err(|err| ConfigError {
            key: key.to_string(),
            value: raw,
            reason: err.to_string(),
        }),
    }
}

fn env_bool(key: &str, default: bool) -> Result<bool, ConfigError> {
    match env_raw(key)? {
        None => Ok(default),
        Some(raw) => match raw.trim().to_ascii_lowercase().as_str() {
            "1" | "true" | "yes" | "on" => Ok(true),
            "0" | "false" | "no" | "off" => Ok(false),
            _ => Err(ConfigError {
                key: key.to_string(),
                value: raw,
                reason: "布尔值只接受 true/false、1/0、yes/no、on/off".to_string(),
            }),
        },
    }
}

/// 把 URL 中 userinfo 部分的密码替换成 `***`，例如
/// `mysql://root:secret@db:3306/app` → `mysql://root:***@db:3306/app`。
fn redact_url(url: &str) -> String {
    let (scheme_end, at) = match (url.find("://"), url.find('@')) {
        (Some(scheme_end), Some(at)) => (scheme_end, at),
        _ => return url.to_string(),
    };
    let userinfo_start = scheme_end + 3;
    if at <= userinfo_start {
        return url.to_string();
    }
    match url[userinfo_start..at].find(':') {
        Some(colon) => format!("{}***{}", &url[..userinfo_start + colon + 1], &url[at..]),
        None => url.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redact_url_hides_password() {
        assert_eq!(
            redact_url("mysql://root:secret@127.0.0.1:3306/app"),
            "mysql://root:***@127.0.0.1:3306/app"
        );
        // 没有密码时原样返回
        assert_eq!(redact_url("mysql://root@host/app"), "mysql://root@host/app");
        // 没有 userinfo、或压根不是 URL 时原样返回
        assert_eq!(redact_url("mysql://host/app"), "mysql://host/app");
        assert_eq!(redact_url("not-a-url"), "not-a-url");
    }

    #[test]
    fn env_helpers_fall_back_to_defaults() {
        // 用几乎不可能存在的变量名，保证测的是「未设置」分支
        assert_eq!(
            env_str("AUTH_CENTER_TEST_UNSET_STR", "fallback").unwrap(),
            "fallback"
        );
        assert_eq!(env_parse("AUTH_CENTER_TEST_UNSET_NUM", 7u16).unwrap(), 7);
        assert!(env_bool("AUTH_CENTER_TEST_UNSET_BOOL", true).unwrap());
    }

    #[test]
    fn env_parse_rejects_bad_value() {
        env::set_var("AUTH_CENTER_TEST_BAD_PORT", "abc");
        let err = env_parse("AUTH_CENTER_TEST_BAD_PORT", 18080u16).unwrap_err();
        assert!(err.to_string().contains("AUTH_CENTER_TEST_BAD_PORT"));
        env::remove_var("AUTH_CENTER_TEST_BAD_PORT");
    }
}
