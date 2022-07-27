use std::env;
use tokio;

mod nubank;

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

    let nubank_client = nubank::NubankClient::new()
        .authenticate(&cert_path, &cpf, &password)
        .await
        .unwrap();
    println!("\n\n\n nubank_client = {:?}", nubank_client);
}
