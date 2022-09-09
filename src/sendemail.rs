use reqwest::StatusCode;

use dotenvy::dotenv;
use std::env;

pub enum Error {
    FailedToSendEmail(),
    BadStatusCode(StatusCode),
}

pub fn send_code(email: &str, code: &i32) -> Result<(), Error> {
    Ok(())
    // let api_key = env::var("SENDGRID_API_KEY").expect("SENDGRID_API_KEY must be set");
    // let from_email = env::var("EMAIL_FROM").expect("EMAIL_FROM must be set");
    // let email_client = SGClient::new(api_key);
    // let content = format!("Your authentication code is: {}", code);
    // let mail_info = Mail::new()
    //     .add_from(&from_email)
    //     .add_to(Destination {
    //         address: email,
    //         name: "My Money",
    //     })
    //     .add_subject("Your authentication code")
    //     .add_content("".to_string(), &content);
    // let response = email_client
    //     .send(mail_info)
    //     .map_err(Error::FailedToSendEmail)?;
    // match response.status() {
    //     StatusCode::OK => Ok(()),
    //     s => Err(Error::BadStatusCode(s)),
    // }
}
