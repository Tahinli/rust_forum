use lettre::{
    message::header::ContentType,
    transport::smtp::{self, authentication::Credentials},
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

use crate::utils::naive_toml_parser;

const MAIL_CONFIG_FILE_LOCATION: &str = "./configs/mail_config.toml";

pub struct MailConfig {
    name: String,
    mail_address: String,
    username: String,
    password: String,
    relay_server: String,
    port: u16,
    starttls: bool,
}

impl Default for MailConfig {
    fn default() -> Self {
        let (header, mut mail_configs) = naive_toml_parser(MAIL_CONFIG_FILE_LOCATION);

        if header == "[mail_config]" {
            Self {
                name: mail_configs.pop_front().unwrap().parse().unwrap(),
                mail_address: mail_configs.pop_front().unwrap().parse().unwrap(),
                username: mail_configs.pop_front().unwrap().parse().unwrap(),
                password: mail_configs.pop_front().unwrap().parse().unwrap(),
                relay_server: mail_configs.pop_front().unwrap().parse().unwrap(),
                port: mail_configs.pop_front().unwrap().parse().unwrap(),
                starttls: mail_configs.pop_front().unwrap().parse().unwrap(),
            }
        } else {
            panic!("Mail Config File Must Include [mail_config] at the First Line")
        }
    }
}

pub async fn send_mail(
    receiver: &String,
    subject: &String,
    body: &String,
) -> Result<smtp::response::Response, smtp::Error> {
    let mail_config = MailConfig::default();

    let message = Message::builder()
        .from(
            format!("{} <{}>", mail_config.name, mail_config.mail_address)
                .parse()
                .unwrap(),
        )
        .to(format!("<{}>", receiver).parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body.to_owned())
        .unwrap();

    let credentials = Credentials::new(
        mail_config.username.to_owned(),
        mail_config.password.to_owned(),
    );

    let mailer = match mail_config.starttls {
        true => AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&mail_config.relay_server),
        false => AsyncSmtpTransport::<Tokio1Executor>::relay(&mail_config.relay_server),
    };

    let mailer = mailer
        .unwrap()
        .credentials(credentials)
        .port(mail_config.port)
        .build();

    mailer.send(message).await
}
