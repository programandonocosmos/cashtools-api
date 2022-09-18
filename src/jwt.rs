use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;

use dotenvy::dotenv;
use std::env;

pub fn generate_token(email: &str) -> String {
    dotenv().ok();
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key: Hmac<Sha256> =
        Hmac::new_from_slice(jwt_secret.as_bytes()).expect("Failed to generate JWT token");
    let mut claims = BTreeMap::new();
    claims.insert("email", email);
    let token_str = claims
        .sign_with_key(&key)
        .expect("Failed to generate token");
    token_str
}

pub fn verify_token(token: String) -> String {
    dotenv().ok();
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key: Hmac<Sha256> =
        Hmac::new_from_slice(jwt_secret.as_bytes()).expect("Failed to generate JWT token");
    let claims: BTreeMap<String, String> = token
        .verify_with_key(&key)
        .expect("Failed to verify JWT token");
    claims["email"].clone()
}
