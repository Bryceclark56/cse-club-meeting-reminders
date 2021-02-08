use serde::{Deserialize, Serialize};
use surf::{self, http::headers};

use crate::constant;

pub async fn get(
    id: &str,
    password: &str,
    site_url: &str,
    auth_token: &str,
) -> surf::Result<MeetingInfo> {
    println!("Retrieving data from Webex");

    let uri = format!(
        "{}{}?siteUrl={}&current=true",
        constant::WEBEX_MEETINGS_API_URL,
        id,
        site_url
    );

    let mut response = surf::get(uri)
        .header(headers::AUTHORIZATION, format!("Bearer {}", auth_token))
        .header("password", password)
        .await?;

    println!("Response status: {}", response.status());

    Ok(response.body_json().await?)
}

/*async fn refresh_auth_token() {

}*/

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeetingInfo {
    id: String,
    meeting_number: String,
    title: String,
    pub agenda: Option<String>,
    password: String,
    meeting_type: MeetingType,
    pub timezone: String,
    pub start: String,          // ISO 8601
    pub end: String,            // ISO 8601
    recurrence: Option<String>, // RFC 2445
    site_url: String,
    pub web_link: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum MeetingType {
    #[serde(rename = "meetingSeries")]
    MeetingSeries,

    #[serde(rename = "scheduledMeeting")]
    ScheduledMeeting,

    Meeting,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum MeetingState {
    Active,
    Scheduled,
    Ready,
    Lobby,
    InProgress,
    Ended,
    Missed,
    Expired,
}
