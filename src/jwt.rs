use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

#[derive(Debug)]
pub enum JwtError {
    FailedToGenerateJwtKey(sha2::digest::InvalidLength),
    FailedToGenerateJwtToken(jwt::Error),
    FailedToVerifyJwtToken(jwt::Error),
}

#[derive(Serialize, Deserialize)]
struct TokenContent {
    email: String,
}

pub fn generate_token(email: &str, jwt_secret: &str) -> Result<String, JwtError> {
    let key: Hmac<Sha256> =
        Hmac::new_from_slice(jwt_secret.as_bytes()).map_err(JwtError::FailedToGenerateJwtKey)?;

    TokenContent {
        email: email.to_string(),
    }
    .sign_with_key(&key)
    .map_err(JwtError::FailedToGenerateJwtToken)
}

pub fn verify_token(token: &str, jwt_secret: &str) -> Result<String, JwtError> {
    let key: Hmac<Sha256> =
        Hmac::new_from_slice(jwt_secret.as_bytes()).map_err(JwtError::FailedToGenerateJwtKey)?;
    let content: TokenContent = token
        .verify_with_key(&key)
        .map_err(JwtError::FailedToVerifyJwtToken)?;
    Ok(content.email)
}

#[cfg(test)]
mod jwt_tests {
    use super::*;

    const SOME_EMAIL: &str = "someemail@gmail.com";
    const GOOD_SECRET: &str = "1234567890987654321";
    const BAD_SECRET: &str = "1234567890987654322";
    const GOOD_TOKEN: &str = "eyJhbGciOiJIUzI1NiJ9.eyJlbWFpbCI6InNvbWVlbWFpbEBnbWFpbC5jb20ifQ.8U6HQs0S3UZtbBwUcz2cJwD7d0C6op5QTMsSB1402ys";
    const BAD_TOKEN: &str = "eyJhbGciOiJIUzI1NiJ9.eyJlbWFpbCI6InNvbWVlbWFpbEBnbWFpbC5jb20ifQ.8U6HQs0S3UZtbBwUcz2cJwD7d0C6op5QTMsSB1402yt";

    #[test]
    fn can_generate_token() {
        let result = generate_token(SOME_EMAIL, GOOD_SECRET).unwrap();
        assert_eq!(result, GOOD_TOKEN);
    }

    #[test]
    fn can_get_email_back() {
        let result = verify_token(GOOD_TOKEN, GOOD_SECRET).unwrap();
        assert_eq!(result, SOME_EMAIL)
    }

    #[test]
    fn cant_verify_bad_token() {
        let result = verify_token(BAD_TOKEN, GOOD_SECRET);
        assert!(result.is_err());
    }

    #[test]
    fn cant_verify_bad_secret() {
        let result = verify_token(GOOD_TOKEN, BAD_SECRET);
        assert!(result.is_err());
    }
}
