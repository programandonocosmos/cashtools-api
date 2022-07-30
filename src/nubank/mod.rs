mod auth;
mod gen_cert;
use std::io::{self};

#[derive(Debug)]
pub enum NubankError {
    AuthError(auth::AuthError),
    GenCertError(gen_cert::GenCertError),
    GenCertBeforeRequestCode(),
}

#[derive(Debug, Clone)]
pub struct NubankClient {
    auth_data: Option<auth::AuthData>,
    code_request_output: Option<gen_cert::CodeRequestOutput>,
    pub sent_to: Option<String>,
}

impl NubankClient {
    pub fn new() -> NubankClient {
        NubankClient {
            auth_data: None,
            code_request_output: None,
            sent_to: None,
        }
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

    pub async fn request_code_to_gen_cert(
        &self,
        cpf: &str,
        password: &str,
    ) -> Result<Self, NubankError> {
        let req_output = gen_cert::request_code(cpf, password)
            .await
            .map_err(NubankError::GenCertError)?;

        let new_client = NubankClient {
            code_request_output: Some(req_output.clone()),
            sent_to: Some(req_output.sent_to),
            ..self.clone()
        };
        Ok(new_client)
    }

    pub async fn gen_certificate(
        &self,
        cert_folder: &str,
        code: &str,
    ) -> Result<String, NubankError> {
        let code_req_output = match self.code_request_output.clone() {
            Some(code_req_output) => code_req_output,
            None => return Err(NubankError::GenCertBeforeRequestCode()),
        };

        let cert_path = gen_cert::exchange_certs(cert_folder, code_req_output, code)
            .await
            .map_err(NubankError::GenCertError)?;

        Ok(cert_path)
    }
}
