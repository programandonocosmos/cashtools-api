use chrono::NaiveDateTime;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use uuid::Uuid;

#[derive(Debug)]
pub enum JwtError {
    FailedToGenerateJwtKey(sha2::digest::InvalidLength),
    FailedToGenerateJwtToken(jwt::Error),
    FailedToVerifyJwtToken(jwt::Error),
    TooOldToken(i64),
}

#[derive(Serialize, Deserialize)]
struct TokenContent {
    id: Uuid,
    time: NaiveDateTime,
}

const EXPIRATION_TIME_IN_DAYS: i64 = 7;

pub fn generate_token(now: NaiveDateTime, id: &Uuid, jwt_secret: &str) -> Result<String, JwtError> {
    let key: Hmac<Sha256> =
        Hmac::new_from_slice(jwt_secret.as_bytes()).map_err(JwtError::FailedToGenerateJwtKey)?;

    TokenContent { id: *id, time: now }
        .sign_with_key(&key)
        .map_err(JwtError::FailedToGenerateJwtToken)
}

pub fn verify_token(now: NaiveDateTime, token: &str, jwt_secret: &str) -> Result<Uuid, JwtError> {
    let key: Hmac<Sha256> =
        Hmac::new_from_slice(jwt_secret.as_bytes()).map_err(JwtError::FailedToGenerateJwtKey)?;
    let TokenContent { id, time } = token
        .verify_with_key(&key)
        .map_err(JwtError::FailedToVerifyJwtToken)?;
    let diff = (now - time).num_days();
    if diff < EXPIRATION_TIME_IN_DAYS {
        Ok(id)
    } else {
        Err(JwtError::TooOldToken(diff))
    }
}

#[cfg(test)]
mod jwt_tests {
    use chrono::Duration;

    use super::*;

    const SOME_UUID: Uuid = Uuid::from_u128(160141200314647599499076565412518613020);
    const NOW: i64 = 1665366563127;
    const GOOD_SECRET: &str = "1234567890987654321";
    const BAD_SECRET: &str = "1234567890987654322";
    const GOOD_TOKEN: &str = "eyJhbGciOiJIUzI1NiJ9.eyJpZCI6Ijc4N2ExMmMzLWU0ZWYtNGY0MC05MzdiLTYyY2JlZThjZDAxYyIsInRpbWUiOiIrNTQ3NDMtMDUtMTRUMjM6MDU6MjcifQ.nEn7aw7oeZi-huDbIQ1oecKI1YpOFK2WJkhcZMQ-4Ik";
    const BAD_TOKEN: &str = "eyJhbGciOiJIUzI1NiJ9.eyJpZCI6Ijc4N2ExMmMzLWU0ZWYtNGY0MC05MzdiLTYyY2JlZThjZDAxYyIsInRpbWUiOiIrNTQ3NDMtMDUtMTRUMjM6MDU6MjcifQ.nEn7aw7oeZi-huDbIQ1oecKI1YpOFK2WJkhcZMQ-4Is";

    fn now() -> NaiveDateTime {
        NaiveDateTime::from_timestamp(NOW, 0)
    }

    #[test]
    fn can_generate_token() {
        let result = generate_token(now(), &SOME_UUID, GOOD_SECRET).unwrap();
        assert_eq!(result, GOOD_TOKEN);
    }

    #[test]
    fn can_get_id_back() {
        let result = verify_token(now(), GOOD_TOKEN, GOOD_SECRET).unwrap();
        assert_eq!(result, SOME_UUID);
    }

    #[test]
    fn cant_verify_bad_token() {
        let result = verify_token(now(), BAD_TOKEN, GOOD_SECRET);
        assert!(result.is_err());
    }

    #[test]
    fn cant_verify_bad_secret() {
        let result = verify_token(now(), GOOD_TOKEN, BAD_SECRET);
        assert!(result.is_err());
    }

    #[test]
    fn cant_verify_too_old_token() {
        let result = verify_token(
            now() + Duration::days(EXPIRATION_TIME_IN_DAYS + 1),
            GOOD_TOKEN,
            GOOD_SECRET,
        );
        assert!(result.is_err());
    }
}
