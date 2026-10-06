use mailchannels_email_api::{
    apis::{
        configuration::Configuration,
        send_api::{send_email, SendEmailSuccess},
    },
    models::{ContentItem, EmailAddress, MailSendBody, Personalization},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key = std::env::var("MAILCHANNELS_API_KEY")?;
    let sender = std::env::var("MAILCHANNELS_SENDER")?;
    let recipient = std::env::var("MAILCHANNELS_RECIPIENT")?;
    let message = MailSendBody::new(
        vec![ContentItem::new(
            "text/plain".into(),
            "Rust SDK dry-run validation".into(),
        )],
        EmailAddress::new(sender),
        vec![Personalization::new(vec![EmailAddress::new(recipient)])],
        "Rust SDK dry run".into(),
    );
    let response = send_email(&Configuration::new(), &key, message, Some(true)).await?;
    match response.entity {
        Some(SendEmailSuccess::Status200(rendered)) => {
            println!(
                "Validated {} rendered message(s)",
                rendered.data.map_or(0, |data| data.len())
            );
        }
        _ => return Err("Unexpected dry-run response".into()),
    }
    Ok(())
}
