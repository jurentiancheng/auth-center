//! JWT 签发与校验（HS256）。
//!
//! 只做签名与验签，不查库、不碰业务。

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// JWT 载荷。
///
/// token 是客户端可读的（base64 明文，未加密），所以只放鉴权必需的最小字段；
/// 密码、身份证号这类敏感信息一律不放 —— 需要更多资料请拿 `sub` 回查。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// 用户主键（`user.id`）
    pub sub: i64,
    pub user_name: String,
    pub is_admin: bool,
    /// 签发时间（unix 秒）
    pub iat: u64,
    /// 过期时间（unix 秒）
    pub exp: u64,
}

/// 签名密钥不进 `Debug`，避免随日志泄漏。
#[derive(Clone)]
pub struct JwtService {
    secret: Vec<u8>,
    expire_seconds: i64,
}

impl JwtService {
    pub fn new(secret: &str, expire_seconds: i64) -> Self {
        Self {
            secret: secret.as_bytes().to_vec(),
            expire_seconds,
        }
    }

    pub fn expire_seconds(&self) -> i64 {
        self.expire_seconds
    }

    pub fn sign(&self, user_id: i64, user_name: &str, is_admin: bool) -> Result<String, String> {
        let now = jsonwebtoken::get_current_timestamp();
        let claims = Claims {
            sub: user_id,
            user_name: user_name.to_string(),
            is_admin,
            iat: now,
            exp: now + self.expire_seconds.max(1) as u64,
        };

        encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(&self.secret),
        )
        .map_err(|err| format!("JWT 签发失败：{err}"))
    }

    pub fn verify(&self, token: &str) -> Result<Claims, String> {
        // `Validation::new(HS256)` 会把可接受算法限定为 HS256 且默认校验 exp。
        // 限定算法这一条很关键：否则攻击者可以把 header 的 alg 改成 none 或换成非对称算法来绕过验签。
        // 载荷里没有 aud，因此不会触发 audience 校验。
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(&self.secret),
            &Validation::new(Algorithm::HS256),
        )
        .map(|data| data.claims)
        .map_err(|err| format!("JWT 校验失败：{err}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SECRET: &str = "unit-test-secret-0123456789abcdefghij";

    fn service(expire_seconds: i64) -> JwtService {
        JwtService::new(SECRET, expire_seconds)
    }

    #[test]
    fn sign_then_verify_roundtrip() {
        let jwt = service(3600);
        let token = jwt.sign(42, "alice", true).expect("签发应成功");

        let claims = jwt.verify(&token).expect("验签应成功");
        assert_eq!(claims.sub, 42);
        assert_eq!(claims.user_name, "alice");
        assert!(claims.is_admin);
        assert_eq!(claims.exp - claims.iat, 3600, "有效期应等于配置的秒数");
    }

    #[test]
    fn tampered_token_is_rejected() {
        let jwt = service(3600);
        let token = jwt.sign(1, "bob", false).unwrap();

        // 只改签名段的最后一个字符
        let mut tampered = token.clone();
        let last = tampered.pop().unwrap();
        tampered.push(if last == 'A' { 'B' } else { 'A' });

        assert!(jwt.verify(&tampered).is_err(), "篡改后的令牌必须被拒");
    }

    #[test]
    fn token_signed_with_another_secret_is_rejected() {
        let mine = service(3600);
        let theirs = JwtService::new("another-secret-0123456789abcdefghij", 3600);
        let token = theirs.sign(1, "bob", false).unwrap();

        assert!(mine.verify(&token).is_err(), "换了密钥必须验签失败");
    }

    #[test]
    fn expired_token_is_rejected() {
        let jwt = service(3600);

        // 手工造一个 exp 已过去的令牌：sign() 会把有效期下限钳到 1 秒，
        // 而 Validation 默认有 60 秒 leeway，靠 sleep 测过期不现实。
        let now = jsonwebtoken::get_current_timestamp();
        let claims = Claims {
            sub: 1,
            user_name: "bob".to_string(),
            is_admin: false,
            iat: now - 7200,
            exp: now - 3600,
        };
        let token = encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(jwt.secret.as_slice()),
        )
        .unwrap();

        assert!(jwt.verify(&token).is_err(), "过期令牌必须被拒");
    }
}
