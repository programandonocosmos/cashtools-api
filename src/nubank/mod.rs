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
        cert_path: &str,
        cpf: &str,
        password: &str,
    ) -> Result<Self, NubankError> {
        let auth_data = auth::authenticate(cert_path, cpf, password)
            .await
            .map_err(NubankError::AuthError)?;

        let new_client = NubankClient {
            auth_data: Some(auth_data),
            ..self.clone()
        };

        Ok(new_client)
    }

    pub async fn generate_certificate(
        cpf: String,
        password: String,
    ) -> Result<String, NubankError> {
        auth::generate_certificate(cpf, password)
            .await
            .map_err(NubankError::AuthError)
    }
}
