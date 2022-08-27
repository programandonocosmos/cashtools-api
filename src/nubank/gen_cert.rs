use openssl::pkcs12::Pkcs12;
use openssl::pkey::{PKey, Private};
use openssl::rsa::Rsa;
use openssl::x509::X509;
use random_string::generate;
use serde::Deserialize;
use serde_json::{self};
use std::collections::HashMap;
use std::fs;
use std::str::{self, Utf8Error};
#[path = "./discovery.rs"]
mod discovery;

#[derive(Debug)]
pub enum GenCertError {
    DiscoveryError(discovery::DiscoveryError),
    FailedToGeneratePrivateKey(openssl::error::ErrorStack),
    FailedToGeneratePublicKey(openssl::error::ErrorStack),
    FailedToConvertPublicKeyToStr(Utf8Error),
    MainRequestFailed(reqwest::Error),
    HeaderKeyNotFound(reqwest::header::HeaderMap),
    FailedToConvertHeaderValueToStr(reqwest::header::ToStrError),
    FailedToReadHeaderValue(String),
    HeaderValueKeyNotFound(reqwest::header::HeaderValue),
    ExchangeCertRequestFailed(reqwest::Error),
    ExchangeCertRequestDecodingFailed(reqwest::Error),
    ExchangeCertJsonConversionFailed(serde_json::Error),
    FailedToGenerateCert(openssl::error::ErrorStack),
    FailedToWriteCert(std::io::Error),
}

#[derive(Debug, Clone)]
pub struct CodeRequestOutput {
    url: String,
    pub sent_to: String,
    encrypted_code: String,
    payload: HashMap<String, String>,
    key1: PKey<Private>,
}

#[derive(Deserialize)]
pub struct ExchangeCertDTO {
    certificate: String,
}

pub async fn request_code(cpf: &str, password: &str) -> Result<CodeRequestOutput, GenCertError> {
    let url = discovery::get_url("gen_certificate".to_string())
        .await
        .map_err(GenCertError::DiscoveryError)?;

    let charset = "abcdefghijklmnopqrstuvwxyz1234567890";
    let device_id = generate(12, charset);

    let key1 = gen_private_key()?;
    let key2 = gen_private_key()?;

    let payload = build_payload(cpf, password, &key1, &key2, device_id)?;

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .json(&payload)
        .send()
        .await
        .map_err(GenCertError::MainRequestFailed)?;

    let headers = response.headers();
    let header_value = match headers.get("WWW-Authenticate") {
        Some(header_value) => header_value,
        None => return Err(GenCertError::HeaderKeyNotFound(headers.to_owned())),
    };
    let parsed_header_value = parse_header_value(header_value)?;
    let (encrypted_code, sent_to) = match (
        parsed_header_value.get("device-authorization_encrypted-code"),
        parsed_header_value.get("sent-to"),
    ) {
        (Some(v1), Some(v2)) => (
            v1.to_string().replace("\"", ""),
            v2.to_string().replace("\"", ""),
        ),
        _ => {
            return Err(GenCertError::HeaderValueKeyNotFound(
                header_value.to_owned(),
            ))
        }
    };

    Ok(CodeRequestOutput {
        url,
        sent_to,
        encrypted_code,
        payload,
        key1,
    })
}

pub async fn exchange_certs(
    cert_folder: &str,
    code_request_output: CodeRequestOutput,
    code: &str,
) -> Result<String, GenCertError> {
    let new_payload = build_new_payload(
        code_request_output.payload.clone(),
        &code_request_output.encrypted_code.clone(),
        code,
    );

    let url = code_request_output.url.clone();

    let client = reqwest::Client::new();
    let response_str = client
        .post(url)
        .json(&new_payload)
        .send()
        .await
        .map_err(GenCertError::ExchangeCertRequestFailed)?
        .text()
        .await
        .map_err(GenCertError::ExchangeCertRequestDecodingFailed)?;

    let key = code_request_output.key1.clone();
    let cert_str = serde_json::from_str::<ExchangeCertDTO>(&*response_str)
        .map_err(GenCertError::ExchangeCertJsonConversionFailed)?
        .certificate;

    let cert_bin = get_cert_bin(&cert_str, &key)?;

    let full_path = format!("{}/{}", cert_folder, "cert.p12");
    let _ = fs::write(&full_path, cert_bin).map_err(GenCertError::FailedToWriteCert)?;
    Ok(full_path)
}

fn gen_private_key() -> Result<PKey<Private>, GenCertError> {
    let rsa = Rsa::generate(2048).map_err(GenCertError::FailedToGeneratePrivateKey)?;
    let private_key = PKey::from_rsa(rsa).map_err(GenCertError::FailedToGeneratePrivateKey);
    private_key
}

fn build_payload(
    cpf: &str,
    password: &str,
    key1: &PKey<Private>,
    key2: &PKey<Private>,
    device_id: String,
) -> Result<HashMap<String, String>, GenCertError> {
    let pub_key1 = get_public_key(key1)?;
    let pub_key2 = get_public_key(key2)?;
    let model = format!("MyMoney Client ({})", device_id);
    let mut payload: HashMap<String, String> = HashMap::new();
    payload.insert("login".to_string(), cpf.to_string());
    payload.insert("password".to_string(), password.to_string());
    payload.insert("public_key".to_string(), pub_key1);
    payload.insert("public_key_crypto".to_string(), pub_key2);
    payload.insert("model".to_string(), model);
    payload.insert("device_id".to_string(), device_id);
    Ok(payload)
}

fn build_new_payload(
    old_payload: HashMap<String, String>,
    encrypted_code: &str,
    code: &str,
) -> HashMap<String, String> {
    let mut new_payload = old_payload.clone();
    new_payload.insert("code".to_string(), code.to_string());
    new_payload.insert("encrypted-code".to_string(), encrypted_code.to_string());
    new_payload
}

fn get_public_key(key: &PKey<Private>) -> Result<String, GenCertError> {
    let pub_key = key
        .public_key_to_pem()
        .map_err(GenCertError::FailedToGeneratePublicKey)?;
    let pub_key_str = str::from_utf8(pub_key.as_slice())
        .map_err(GenCertError::FailedToConvertPublicKeyToStr)?
        .to_string();
    Ok(pub_key_str)
}

fn parse_header_value(
    header_value: &reqwest::header::HeaderValue,
) -> Result<HashMap<String, String>, GenCertError> {
    let mut parsed_header_value: HashMap<String, String> = HashMap::new();
    let header_value_str = header_value
        .to_str()
        .map_err(GenCertError::FailedToConvertHeaderValueToStr)?;
    let chunks = header_value_str.split(",");
    for chunk in chunks {
        let mut items = chunk.split("=");
        let (key, value) = match items.clone().count() {
            2 => (items.next().unwrap(), items.next().unwrap()),
            _ => {
                return Err(GenCertError::FailedToReadHeaderValue(
                    header_value_str.to_string(),
                ))
            }
        };
        let parsed_key = key.trim().replace(" ", "_");
        let parsed_value = value.replace(" ", "_");
        parsed_header_value.insert(parsed_key, parsed_value);
    }
    Ok(parsed_header_value)
}

fn get_cert_bin(cert_str: &str, key: &PKey<Private>) -> Result<Vec<u8>, GenCertError> {
    let cert = X509::from_pem(cert_str.as_bytes()).map_err(GenCertError::FailedToGenerateCert)?;

    let pk12_cert = Pkcs12::builder()
        .build("", "", key, &cert)
        .map_err(GenCertError::FailedToGenerateCert)?;

    let der = pk12_cert
        .to_der()
        .map_err(GenCertError::FailedToGenerateCert)?;
    Ok(der)
}
