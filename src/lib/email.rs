use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

#[allow(dead_code)]
pub fn send_email() {

    let email = Message::builder()
        .from("Dimitris <dkl1994@gmail.com>".parse().unwrap())
        .reply_to("Dimitris <dkl1994@gmail.com>".parse().unwrap())
        .to("Jim <dkloromis@gmail.com>".parse().unwrap())
        .subject("Happy new year")
        .body(String::from("Be happy!"))
        .unwrap();

    let creds = Credentials::new("dkl1994@gmail.com".to_string(), "u*e'K8rMs~@-7Gwh".to_string());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}