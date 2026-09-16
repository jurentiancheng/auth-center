//! 密码哈希（argon2id，PHC 字符串格式 `$argon2id$v=19$...`）。
//!
//! 哈希串自带随机盐与参数，所以 `user.password` 列不需要额外的盐字段 ——
//! 直接存 [`hash_password`] 的返回值即可。此前该列存的是明文。

use argon2::password_hash::{phc::PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::Argon2;

/// 明文 -> PHC 哈希串。每次都生成新的随机盐，所以同一密码两次哈希结果不同。
pub fn hash_password(plain: &str) -> Result<String, String> {
    Argon2::default()
        .hash_password(plain.as_bytes())
        .map(|hash| hash.to_string())
        .map_err(|err| format!("密码哈希失败：{err}"))
}

/// 校验明文是否匹配哈希串。
///
/// 哈希串本身非法时（历史明文数据、字段被截断）返回 false 并记警告 ——
/// 对外与「密码错误」不做区分，避免把存储格式问题变成可探测的信号。
pub fn verify_password(plain: &str, password_hash: &str) -> bool {
    match PasswordHash::new(password_hash) {
        Ok(parsed) => Argon2::default()
            .verify_password(plain.as_bytes(), &parsed)
            .is_ok(),
        Err(err) => {
            tracing::warn!("password 列不是合法的 PHC 哈希串，按校验失败处理：{err}");
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_then_verify_roundtrip() {
        let phc = hash_password("correct horse battery staple").expect("哈希应成功");

        assert!(phc.starts_with("$argon2"), "应是 argon2 PHC 串，实际：{phc}");
        assert!(verify_password("correct horse battery staple", &phc));
    }

    #[test]
    fn wrong_password_is_rejected() {
        let phc = hash_password("right-password").unwrap();

        assert!(!verify_password("wrong-password", &phc));
    }

    #[test]
    fn hash_never_contains_plaintext() {
        let plain = "my-secret-password";
        let phc = hash_password(plain).unwrap();

        assert_ne!(phc, plain);
        assert!(!phc.contains(plain), "哈希串里不能出现明文");
    }

    #[test]
    fn same_password_hashes_differently_each_time() {
        // 每次随机盐：相同明文两次哈希结果不同，彩虹表无效
        let a = hash_password("same").unwrap();
        let b = hash_password("same").unwrap();

        assert_ne!(a, b);
        assert!(verify_password("same", &a) && verify_password("same", &b));
    }

    #[test]
    fn invalid_hash_string_is_a_mismatch_not_a_panic() {
        // 历史明文数据 / 空值：不能 panic，更不能误判为通过
        assert!(!verify_password("whatever", "not-a-phc-string"));
        assert!(!verify_password("whatever", ""));
    }
}
