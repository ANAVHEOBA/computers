use lettre::{
    transport::smtp::{
        authentication::{Credentials, Mechanism},
        SmtpTransport,
    },
    transport::smtp::client::{Tls, TlsParameters},
    Message, Transport,
};
use std::env;
use dotenv::dotenv;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    println!("Attempting to send a test email...");

    let smtp_server = env::var("SMTP_SERVER").expect("SMTP_SERVER must be set");
    let smtp_port: u16 = env::var("SMTP_PORT")
        .expect("SMTP_PORT must be set")
        .parse()
        .expect("SMTP_PORT must be a valid number");
    let smtp_login = env::var("SMTP_LOGIN").expect("SMTP_LOGIN must be set");
    let smtp_key = env::var("SMTP_KEY").expect("SMTP_KEY must be set");

    let from_email = "test@example.com"; // Using a simple from address for the test
    let to_email = "wisdomvolt@gmail.com"; // The recipient

    let email = Message::builder()
        .from(from_email.parse().unwrap())
        .to(to_email.parse().unwrap())
        .subject("SMTP Test Email")
        .body("This is a test email to check SMTP configuration.".to_string())
        .unwrap();

    let creds = Credentials::new(smtp_login, smtp_key);

    // Create the TlsParameters for STARTTLS
    let tls_parameters = TlsParameters::new(smtp_server.clone())?;

    let mailer = SmtpTransport::relay(&smtp_server)
        .unwrap()
        .port(smtp_port)
        .tls(Tls::Required(tls_parameters)) // Correctly construct Tls::Required
        .credentials(creds)
        .authentication(vec![Mechanism::Plain])
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => eprintln!("Failed to send email: {:?}", e),
    }

    Ok(())
}
