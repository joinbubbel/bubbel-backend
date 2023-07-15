use lettre::{
    address::Address,
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::{Credentials, Mechanism},
    Message, SmtpTransport, Transport,
};

pub fn send_verify_account_email(
    from: &str,
    from_password: &str,
    email: &str,
    code: &str,
    user_id: &str,
) -> Result<(), ()> {
    let m = Message::builder()
        .to(Mailbox::new(
            None,
            email.parse::<Address>().map_err(|_| ())?,
        ))
        .from(Mailbox::new(None, from.parse::<Address>().unwrap()))
        .subject("Bubbel Account Verication Code")
        .header(ContentType::TEXT_PLAIN)
        .body(format!("user_id: {}\ncode: {}\n", user_id, code))
        .unwrap();

    let sender = SmtpTransport::starttls_relay("smtp-mail.outlook.com")
        .unwrap()
        .credentials(Credentials::new(from.to_owned(), from_password.to_owned()))
        .port(587)
        .authentication(vec![Mechanism::Login])
        .build();

    sender.send(&m).map(|_| ()).map_err(|_| ())
}
