use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};

use reqwest;
use serde::Deserialize;
use serde_json::{self, json};

#[path = "./custom_request_builder.rs"]
mod custom_request_builder;
use custom_request_builder::*;

#[path = "../utils/mod.rs"]
mod utils;

#[path = "./discovery.rs"]
mod discovery;

#[derive(Debug)]
pub enum AuthError {
    DiscoveryError(discovery::DiscoveryError),
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
pub struct AuthDataDTO {
    access_token: String,
    #[serde(rename = "_links")]
    links: HashMap<String, Href>,
}

#[derive(Debug, Clone)]
pub struct AuthData {
    access_token: String,
    feed_url: String,
    bills_url: String,
    customer_url: String,
    query_url: String,
    revoke_token_url: String,
}

pub async fn authenticate(path: &str, cpf: &str, password: &str) -> Result<AuthData, AuthError> {
    let url = discovery::get_url("token".to_string())
        .await
        .map_err(AuthError::DiscoveryError)?;

    let id = get_identity(path)?;
    let client = build_client(id)?;

    let payload = build_payload(cpf, password);

    let response = make_auth_request(client, url, payload).await?;
    let auth_data_dto = read_request_output(response)?;
    let auth_data = build_auth_data_obj(auth_data_dto);

    auth_data
}

fn get_identity(path: &str) -> Result<reqwest::Identity, AuthError> {
    let mut buf = Vec::new();
    let _ = File::open(path)
        .map_err(AuthError::CannotReadCertificate)?
        .read_to_end(&mut buf);

    reqwest::Identity::from_pkcs12_der(&buf, "").map_err(AuthError::CannotInterpretCertificate)
}

fn build_client(id: reqwest::Identity) -> Result<reqwest::Client, AuthError> {
    reqwest::Client::builder()
        .identity(id)
        .build()
        .map_err(AuthError::CannotBuildClient)
}

fn build_payload(cpf: &str, password: &str) -> String {
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
    client
        .post(url)
        .apply_default_header()
        .body(payload)
        .send()
        .await
        .map_err(AuthError::AuthRequestFailed)?
        .text()
        .await
        .map_err(AuthError::AuthRequestDecodingFailed)
}
fn read_request_output(result: String) -> Result<AuthDataDTO, AuthError> {
    serde_json::from_str::<AuthDataDTO>(&*result).map_err(AuthError::AuthJsonConversionFailed)
}

fn build_auth_data_obj(auth_data_dto: AuthDataDTO) -> Result<AuthData, AuthError> {
    match (
        utils::first_or(
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
        _ => Err(AuthError::RequiredFieldsNotFoundInAuthJson(auth_data_dto)),
    }
}
