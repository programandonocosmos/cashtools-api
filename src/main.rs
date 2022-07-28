use std::env;
use tokio;

mod nubank;

mod simple_user_input {
    use std::io;
    pub fn get_input(prompt: &str) -> String {
        println!("{}", prompt);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {}
            Err(_no_updates_is_fine) => {}
        }
        input.trim().to_string()
    }
}
use simple_user_input::get_input;

#[tokio::main]
async fn main() {
    let (cert_folder, cpf, password) = match (
        env::var("CERT_FOLDER"),
        env::var("CPF"),
        env::var("NUBANK_PASSWORD"),
    ) {
        (Ok(cert_folder), Ok(cpf), Ok(password)) => (cert_folder, cpf, password),
        _ => panic!("Missing CERT_FOLDER, CPF or NUBANK_PASSWORD env variables!!!"),
    };

    let cert_generator = nubank::NubankClient::new()
        .request_code_to_gen_cert(&cpf, &password)
        .await
        .unwrap();

    println!("sent to: {:?}", cert_generator.sent_to);
    let code = get_input("Type the code: ");

    let cert_path = cert_generator
        .gen_certificate(&cert_folder, &code)
        .await
        .unwrap();

    println!("Certificate generated: {}", &cert_path);

    let nubank_client = nubank::NubankClient::new()
        .authenticate(&cert_path, &cpf, &password)
        .await
        .unwrap();
    println!("\n\n\n nubank_client = {:?}", nubank_client);
}
