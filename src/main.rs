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
    let cpf = get_input("CPF: ");
    let password = get_input("Nubank password: ");

    let cert_generator = nubank::NubankClient::new()
        .request_code_to_gen_cert(&cpf, &password)
        .await
        .unwrap();

    println!("sent to: {:?}", cert_generator.sent_to);
    let code = get_input("Type the code: ");

    let cert_folder = get_input("Folder to write the certificate: ");

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
