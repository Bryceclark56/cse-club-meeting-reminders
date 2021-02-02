use std::fmt::Display;
use chrono::Utc;
use chrono::Date;
use hyper::{Body, Method, Request, Uri, Client};
use hyper_tls::HttpsConnector;
use serde::Serialize;


pub async fn send_webhook(webhook_url: Uri, webhook: &Webhook) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Sending webhook to Discord");

    let request = Request::builder()
        .method(Method::POST)
        .uri(webhook_url)
        .header(hyper::header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(webhook)?))?;

    let https = HttpsConnector::new();
    let client = Client::builder()
        .build::<_, hyper::Body>(https);

    let response = client.request(request).await?;

    println!("Reponse: {}", response.status());

    Ok(())
}

#[derive(Serialize)]
pub struct Webhook {
    embeds: Vec<Embed> // We only use this field for now
}

impl Webhook {
    pub fn meeting_reminder(meeting_date: &Date<Utc>, agenda: &str, webex_link: &str, webex_password: &str, color: i32) -> Webhook {
        Webhook {
            embeds: vec![Embed::meeting_reminder(meeting_date, agenda, webex_link, webex_password, color)]
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize)]
struct Embed {
    title: Option<String>,
    #[serde(rename = "type")]
    embed_type: String,
    description: Option<String>,
    //url: Option<String>,
    //timestamp: Option<String>, // ISO8601 timestamp / RFC3339
    color: Option<i32>,
    footer: Option<EmbedFooter>,
    //image: Option<EmbedImage>,
    //thumbnail: Option<EmbedThumbnail>,
    //video: Option<EmbedVideo>,
    //provider: Option<EmbedProvider>,
    //author: Option<EmbedAuthor>,
    fields: Option<Vec<EmbedField>>
}

impl Embed {
    fn meeting_reminder(meeting_date: &Date<Utc>, agenda: &str, webex_link: &str, webex_password: &str, color: i32) -> Embed {
        Embed {
            title: Some(format!("Upcoming Meeting - {}", meeting_date.format("%-m/%-d"))),
            embed_type: EmbedType::Rich.value(),
            description: Some(String::from("CSE Club weekly meeting starting in 15 minutes!")),
            fields: Some(vec![
                EmbedField{
                    name: String::from("Agenda:"),
                    value: agenda.to_string(),
                    inline: Some(false)
                },
                EmbedField{
                    name: String::from("Webex Link:"),
                    value: webex_link.to_string(),
                    inline: Some(false)
                },
                EmbedField{
                    name: String::from("Password:"),
                    value: webex_password.to_string(),
                    inline: Some(false)
                }
            ]),
            color: Some(color),
            footer: Some(EmbedFooter{text:String::from("Everyone is welcome to join!")}),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum EmbedType {
    Rich, // Only one we actually use for webhooks
    Image,
    Video,
    Gif,
    Article,
    Link
}

impl EmbedType {
    pub fn value(&self) -> String {
        self.to_string().to_lowercase()
    }
}

impl Display for EmbedType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

#[derive(Debug, PartialEq, Eq, Serialize)]
struct EmbedFooter {
    text: String
}

/*struct EmbedImage {

}

struct EmbedThumbnail {

}

struct EmbedVideo {

}

struct EmbedProvider {

}

struct EmbedAuthor {

}*/

#[derive(Debug, PartialEq, Eq, Serialize)]
struct EmbedField {
    name: String,
    value: String,
    inline: Option<bool>
}

#[cfg(test)]
mod tests {
    use std::vec;

    use chrono::{TimeZone, Utc};

    use rand::{Rng, distributions::Alphanumeric};

    use crate::constant;
    use super::{Embed, EmbedType, EmbedField, EmbedFooter};

    #[test]
    fn embed_type_value_is_lowercase_name() {
        let eapairs = vec![
            (EmbedType::Rich.value(), "rich"),
            (EmbedType::Image.value(), "image"),
            (EmbedType::Video.value(), "video"),
            (EmbedType::Gif.value(), "gif"),
            (EmbedType::Article.value(), "article"),
            (EmbedType::Link.value(), "link")
        ];

        for (actual, expected) in eapairs {
            assert_eq!(actual, expected, "Should return lowercase version of enum name.");
        }
    }

    fn random_string(length: usize) -> String {
        rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
    }

    #[test]
    fn embed_meeting_reminder_returns_correct_struct() {
        let meeting_date_str: &str = "1/27";
        let meeting_datetime = Utc.ymd(2021, 1, 27);

        let fake_agenda = random_string(20);
        let fake_link = random_string(30);
        let fake_password = random_string(15);

        let expected = Embed {
            title: Some(format!("Upcoming Meeting - {}", meeting_date_str)),
            embed_type: EmbedType::Rich.value(),
            description: Some("CSE Club weekly meeting starting in 15 minutes!".to_string()),
            fields: Some(vec![
                EmbedField{
                    name: "Agenda:".to_string(),
                    value: fake_agenda.clone(),
                    inline: Some(false)
                },
                EmbedField{
                    name: "Webex Link:".to_string(),
                    value: fake_link.clone(),
                    inline: Some(false)
                },
                EmbedField{
                    name: "Password:".to_string(),
                    value: fake_password.to_string(),
                    inline: Some(false)
                }
            ]),
            color: Some(constant::EMBED_COLOR),
            footer: Some(EmbedFooter{text:"Everyone is welcome to join!".to_string()}),
        };

        let actual = Embed::meeting_reminder(&meeting_datetime, &fake_agenda, &fake_link, &fake_password, constant::EMBED_COLOR);
        assert_eq!(expected, actual);

        let actual = Embed::meeting_reminder(&meeting_datetime, &fake_agenda, &fake_link, &random_string(16), constant::EMBED_COLOR);
        assert_ne!(expected, actual);
    }
}