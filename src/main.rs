use std::env;

extern crate hyper;
use hyper::Uri;

extern crate chrono;

use chrono::Local;

mod constant;
mod discord;
mod webex;
use webex::meeting;

#[tokio::main]
async fn main() -> Result<(), Box<(dyn std::error::Error + Send + Sync + 'static)>>{
    println!("Running meeting reminder");

    let meeting_id = env::var("WEBEX_MEETING_ID")?;
    let meeting_password = env::var("WEBEX_MEETING_PASSWORD")?;
    let webex_token = env::var("WEBEX_AUTH_TOKEN")?;

    let meeting_info = meeting::get(&meeting_id, &meeting_password, constant::WEBEX_SITE_URL, &webex_token).await?;

    let webhook_uri: Uri = env::var("DISCORD_WEBHOOK_URL")?.parse()?;
    let meeting_date = Local::today();
    let agenda = meeting_info.agenda.unwrap_or_else(|| "none".to_string());
    let webex_link = meeting_info.web_link;
    let webex_password = meeting_password;
    let color = constant::EMBED_COLOR;

    let webhook = discord::Webhook::meeting_reminder(&meeting_date, &agenda, &webex_link, &webex_password, color);

    discord::send_webhook(webhook_uri, &webhook).await
}