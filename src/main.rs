use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, Read};

use reqwest;
use serde::Deserialize;
use serde_json::{self, json, Value};
use tokio;

mod custom_request_builder;
use custom_request_builder::*;

#[derive(Debug)]
enum AuthError {
    DiscoveryRequestFailed(reqwest::Error),
    DiscoveryRequestDecodingFailed(reqwest::Error),
    DiscoveryJsonConversionFailed(serde_json::Error),
    UrlNameNotFoundInDiscoveryJson(String),
    CannotReadCertificate(io::Error),
    CannotInterpretCertificate(reqwest::Error),
    CannotBuildClient(reqwest::Error),
    AuthRequestFailed(reqwest::Error),
    AuthRequestDecodingFailed(reqwest::Error),
    AuthJsonConversionFailed(serde_json::Error),
    RequiredFieldsNotFoundInAuthJson(AuthDataDTO),
}

#[derive(Deserialize, Debug, Clone)]
struct Href {
    href: String,
}

#[derive(Deserialize, Debug)]
struct AuthDataDTO {
    access_token: String,
    #[serde(rename = "_links")]
    links: HashMap<String, Href>,
}

#[derive(Debug)]
struct AuthData {
    access_token: String,
    feed_url: String,
    bills_url: String,
    customer_url: String,
    query_url: String,
    revoke_token_url: String,
}

#[tokio::main]
async fn main() {
    let (cert_path, cpf, password) = match (
        env::var("CERT_PATH"),
        env::var("CPF"),
        env::var("NUBANK_PASSWORD"),
    ) {
        (Ok(cert_path), Ok(cpf), Ok(password)) => (cert_path, cpf, password),
        _ => panic!("Missing NUBANK_PASSWORD env variable!!!"),
    };

    let auth_data = auth(cert_path, cpf, password).await.unwrap();
    println!("\n\n\nauth_data = {:?}", auth_data);
}

async fn auth(path: String, cpf: String, password: String) -> Result<AuthData, AuthError> {
    let url = get_url("token".to_string()).await?;

    let id = get_identity(path)?;
    let client = build_client(id)?;

    let payload = build_payload(cpf, password);

    let request_output = make_auth_request(client, url, payload).await?;
    let auth_data_dto = read_request_output(request_output)?;
    let auth_data = build_auth_data_obj(auth_data_dto)?;

    Ok(auth_data)
}

async fn get_url(name: String) -> Result<String, AuthError> {
    let discovery_app_url = "https://prod-s0-webapp-proxy.nubank.com.br/api/app/discovery";

    let urls_str = reqwest::get(discovery_app_url)
        .await
        .map_err(|x| AuthError::DiscoveryRequestFailed(x))?
        .text()
        .await
        .map_err(|x| AuthError::DiscoveryRequestDecodingFailed(x))?;

    let urls = serde_json::from_str::<Value>(&*urls_str)
        .map_err(|x| AuthError::DiscoveryJsonConversionFailed(x))?;

    match urls.get(&name) {
        Some(v) => Ok(v.to_string().replace("\"", "")),
        _ => return Err(AuthError::UrlNameNotFoundInDiscoveryJson(name)),
    }
}

fn get_identity(path: String) -> Result<reqwest::Identity, AuthError> {
    let mut buf = Vec::new();
    let _ = File::open(path)
        .map_err(|x| AuthError::CannotReadCertificate(x))?
        .read_to_end(&mut buf);
    let id = reqwest::Identity::from_pkcs12_der(&buf, "")
        .map_err(|x| AuthError::CannotInterpretCertificate(x))?;
    Ok(id)
}

fn build_client(id: reqwest::Identity) -> Result<reqwest::Client, AuthError> {
    reqwest::Client::builder()
        .identity(id)
        .build()
        .map_err(|x| AuthError::CannotBuildClient(x))
}

fn build_payload(cpf: String, password: String) -> String {
    json!(
        {
            "grant_type": "password",
            "client_id": "legacy_client_id",
            "client_secret": "legacy_client_secret",
            "login": cpf,
            "password": password
        }
    )
    .to_string()
}

async fn make_auth_request(
    client: reqwest::Client,
    url: String,
    payload: String,
) -> Result<String, AuthError> {
    let result = client
        .post(url)
        .apply_default_header()
        .body(payload)
        .send()
        .await
        .map_err(|x| AuthError::AuthRequestFailed(x))?
        .text()
        .await
        .map_err(|x| AuthError::AuthRequestDecodingFailed(x))?;

    Ok(result)
}
fn read_request_output(result: String) -> Result<AuthDataDTO, AuthError> {
    let obj = serde_json::from_str::<AuthDataDTO>(&*result)
        .map_err(|x| AuthError::AuthJsonConversionFailed(x))?;

    Ok(obj)
}

fn build_auth_data_obj(auth_data_dto: AuthDataDTO) -> Result<AuthData, AuthError> {
    match (
        first_or(
            auth_data_dto.links.get("events"),
            auth_data_dto.links.get("magnitude"),
        ),
        auth_data_dto.links.get("bills_summary"),
        auth_data_dto.links.get("customer"),
        auth_data_dto.links.get("ghostflame"),
        auth_data_dto.links.get("revoke_token"),
    ) {
        (
            Some(feed_url),
            Some(bills_url),
            Some(customer_url),
            Some(query_url),
            Some(revoke_token_url),
        ) => Ok(AuthData {
            access_token: auth_data_dto.access_token,
            feed_url: feed_url.clone().href,
            bills_url: bills_url.clone().href,
            customer_url: customer_url.clone().href,
            query_url: query_url.clone().href,
            revoke_token_url: revoke_token_url.clone().href,
        }),
        _ => return Err(AuthError::RequiredFieldsNotFoundInAuthJson(auth_data_dto)),
    }
}

fn first_or<T>(a: Option<T>, b: Option<T>) -> Option<T> {
    match a {
        Some(v) => Some(v),
        None => b,
    }
}
