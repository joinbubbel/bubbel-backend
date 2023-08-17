use super::*;
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
) -> Result<(), ()> {
    trace!("Sending verify email to {}.", email);
    let image_url =
        "https://res.cloudinary.com/dxmrcocqb/image/upload/v1690213642/Group_1210_akkvya.png";
    let m = Message::builder()
        .to(Mailbox::new(
            None,
            email.parse::<Address>().map_err(|_| ())?,
        ))
        .from(Mailbox::new(None, from.parse::<Address>().unwrap()))
        .subject(format!("Bubbel Account Verication Code {}", code))
        .header(ContentType::TEXT_HTML)
        .body(format!( r#"
        <html>
        <body style="padding:0.5rem 1rem">
        <img src="{}" alt="Bubbel Logo" style="width: 2.5rem" />
            <p>Hi 👋,</p>
            <p>You requested a verification code to sign up for Bubbel. It's here:</p>
            <p style="font-size: 18px;">
            <h3>{}</h3>
            </p>
            <p>It will expire in 15 minutes.</p>
            <p>Tip: you can triple-click the box to copy-paste the whole thing, including the dash in the middle.</p>
            <p>- Bubbel</p>
        </body>
        </html>
        "#,
        image_url, code
))
        .unwrap();

    let sender = SmtpTransport::starttls_relay("smtp-mail.outlook.com")
        .unwrap()
        .credentials(Credentials::new(from.to_owned(), from_password.to_owned()))
        .port(587)
        .authentication(vec![Mechanism::Login])
        .build();

    sender.send(&m).map(|_| ()).map_err(|_| ())
}
