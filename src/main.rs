use std::env;

use tide::prelude::*;

extern crate chrono;

use chrono::Local;

mod constant;
mod discord;
mod webex;

#[async_std::main]
async fn main() -> tide::Result<()> {
    println!("Starting handler!");
    get_func_env_vars()?; // Tests to ensure they exist at runtime

    let listen_url = format!("127.0.0.1:{}", env::var("FUNCTIONS_HTTPWORKER_PORT")?);
    let mut handler = tide::new();

    handler.at("/cse-meeting-reminder").post(|_| async {
        send_reminder().await?;
        Ok(json!({}))
    });
    handler.listen(listen_url).await?;

    println!("Closing handler");
    Ok(())
}

async fn send_reminder() -> tide::Result<()> {
    println!("===Running meeting reminder===");

    let (meeting_id, meeting_password, webex_token, webhook_uri) = get_func_env_vars()?;

    let meeting_info = webex::meeting::get(
        &meeting_id,
        &meeting_password,
        constant::WEBEX_SITE_URL,
        &webex_token,
    )
    .await?;

    let meeting_date = Local::today();
    let agenda = meeting_info.agenda.unwrap_or_else(|| String::from("None"));
    let webex_link = meeting_info.web_link;
    let webex_password = meeting_password;
    let color = constant::EMBED_COLOR;

    discord::Webhook::meeting_reminder(&meeting_date, &agenda, &webex_link, &webex_password, color)
        .send(&webhook_uri)
        .await?;

    println!("===========Finished===========");

    Ok(())
}

// Retrieves the environment variables used in
// send_reminder()
fn get_func_env_vars() -> Result<(String, String, String, String), env::VarError> {
    Ok((
        env::var("WEBEX_MEETING_ID")?,
        env::var("WEBEX_MEETING_PASSWORD")?,
        env::var("WEBEX_AUTH_TOKEN")?,
        env::var("DISCORD_WEBHOOK_URL")?,
    ))
}
