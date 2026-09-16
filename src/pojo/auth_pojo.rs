use serde::{Deserialize, Serialize};

/// 登录入参。
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoginDto {
    pub user_name: String,
    pub password: String,
}

/// 登录成功返回。
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoginVo {
    /// 访问令牌
    pub token: String,
    /// 固定为 `Bearer`，方便客户端直接拼 `Authorization` 头
    pub token_type: String,
    /// 有效期（秒）
    pub expires_in: i64,
    pub user_id: i64,
    pub user_name: String,
    pub is_admin: bool,
}
