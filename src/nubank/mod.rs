mod auth;
use std::io::{self};

#[derive(Debug)]
pub enum NubankError {
    AuthError(auth::AuthError),
    RequestError(reqwest::Error),
    JsonError(serde_json::Error),
    IoError(io::Error),
}

#[derive(Debug, Clone)]
pub struct NubankClient {
    auth_data: Option<auth::AuthData>,
}

impl NubankClient {
    pub fn new() -> NubankClient {
        NubankClient { auth_data: None }
    }

    pub async fn authenticate(
        &self,
        cert_path: String,
        cpf: String,
        password: String,
    ) -> Result<Self, NubankError> {
        let auth_data = auth::authenticate(cert_path, cpf, password)
            .await
            .map_err(auth_err_to_nubank_err)?;
        let mut new_client = self.clone();
        new_client.auth_data = Some(auth_data);
        Ok(new_client)
    }

    pub async fn generate_certificate(
        cpf: String,
        password: String,
    ) -> Result<String, NubankError> {
        auth::generate_certificate(cpf, password)
            .await
            .map_err(auth_err_to_nubank_err)
    }
}

fn auth_err_to_nubank_err(auth_err: auth::AuthError) -> NubankError {
    NubankError::AuthError(auth_err)
}
