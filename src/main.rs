use std::env;

use tide::Request;
use tide::prelude::*;

extern crate chrono;

use chrono::Local;

mod constant;
mod discord;
mod webex;
use webex::meeting;

#[async_std::main]
async fn main() -> tide::Result<()>{
    println!("Starting handler!");

    let listen_url = format!("127.0.0.1:{}", env::var("FUNCTIONS_HTTPWORKER_PORT")?);
    let mut handler = tide::new();

    handler.at("/cse-meeting-reminder").post(|_| async {
        send_reminder().await; // We should check the error
        Ok(json!({}))
    });

    handler.listen(listen_url).await?;
    println!("Closing handler");
    Ok(())
}

async fn send_reminder() -> Result<(), Box<(dyn std::error::Error + Send + Sync)>> {
    println!("======Running meeting reminder======");

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

    discord::send_webhook(webhook_uri, &webhook).await?;

    println!("======Finished======");

    Ok(())
}