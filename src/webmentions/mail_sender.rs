use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;

use crate::models::Mention;

pub fn send_mail(mention: Mention) {
    let host = env::var("MAIL_HOST").expect("MAIL_HOST is not set");
    let mail_user = env::var("MAIL_USER").expect("MAIL_USER is not set");
    let mail_pass = env::var("MAIL_PASS").expect("MAIL_PASS is not set");

    let body = format!(
        "Hi,

new Mention reveived,
from: {}
to: {}

Regards,
 WWWTech",
        mention.source_url, mention.target_url
    );

    let email = Message::builder()
        .from("Christian Kruse <christian@kruse.cool>".parse().unwrap())
        .to("Christian Kruse <christian@kruse.cool>".parse().unwrap())
        .subject("New Mention")
        .body(body)
        .unwrap();

    let creds = Credentials::new(mail_user, mail_pass);
    let mailer = SmtpTransport::relay(host.as_str()).unwrap().credentials(creds).build();

    let _ = mailer.send(&email);
}
