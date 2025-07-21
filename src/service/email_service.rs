use lettre::{
    transport::smtp::{
        authentication::Credentials,
        SmtpTransport,
    },
    transport::smtp::client::{Tls, TlsParameters},
    Message, Transport,
};
use std::env;

/// Sends an email using the configured Gmail SMTP server.
pub fn send_email(to: &str, to_name: &str, subject: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
    let smtp_server = env::var("SMTP_SERVER").expect("SMTP_SERVER must be set");
    let smtp_port: u16 = env::var("SMTP_PORT").expect("SMTP_PORT must be set").parse()?;
    let smtp_user = env::var("SMTP_USER").expect("SMTP_USER must be set");
    let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");

    let from_name = env::var("FROM_NAME").unwrap_or_else(|_| "Account Verification".to_string());

    let email = Message::builder()
        .from(format!("{} <{}>", from_name, smtp_user).parse()?)
        .to(format!("{} <{}>", to_name, to).parse()?)
        .subject(subject)
        .header(lettre::message::header::ContentType::TEXT_HTML)
        .body(body.to_string())?;

    let creds = Credentials::new(smtp_user, smtp_password);
    
    let tls_parameters = TlsParameters::new(smtp_server.clone())?;

    let mailer = SmtpTransport::relay(&smtp_server)?
        .port(smtp_port)
        .tls(Tls::Required(tls_parameters))
        .credentials(creds)
        .build();

    mailer.send(&email)?;

    println!("Email sent successfully via Gmail SMTP to {}", to);

    Ok(())
} 