/* Accessing the Webex REST API */
pub mod meeting {
    use std::str::FromStr;

    use hyper::body;
    use hyper::{Body, Uri, Client, Request};
    use hyper::header::AUTHORIZATION;
    use hyper_tls::HttpsConnector;
    use serde::{Serialize, Deserialize};

    use crate::constant;

    pub async fn get(id: &str, password: &str, site_url: &str, auth_token: &str) -> Result<MeetingInfo, Box<dyn std::error::Error + Send + Sync>> {
        println!("Getting data from Webex!");

        let uri = Uri::from_str(&format!("{}{}?siteUrl={}&current=true", constant::WEBEX_MEETINGS_API_URL, id, site_url))?;

        let request = Request::get(uri)
            .header(&AUTHORIZATION, format!("Bearer {}", auth_token))
            .header("password", password)
            .body(Body::empty())?;

        let https = HttpsConnector::new();
        let client = Client::builder()
            .build::<_, hyper::Body>(https);

        let mut response = client.request(request).await?;
        println!("Response: {}", response.status());

        let body = body::to_bytes(response.body_mut()).await?;

        let body_vec = body.to_vec();
        let string_body = std::str::from_utf8(&body_vec[..])?;
        println!("{}", string_body);

        Ok(serde_json::from_str(string_body)?)
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct MeetingInfo {
        id: String,
        meeting_number: String,
        title: String,
        pub agenda: Option<String>,
        password: String,
        phone_and_video_system_password: Option<String>,
        meeting_type: MeetingType,
        pub timezone: String,
        pub start: String, // ISO 8601
        pub end: String, // ISO 8601
        recurrence: String, // RFC 2445
        host_user_id: String,
        host_display_name: String,
        host_email: String,
        host_key: Option<String>,
        site_url: String,
        pub web_link: String,
        sip_address: String,
        dial_in_ip_address: String,
        enabled_auto_record_meeting: bool,
        allow_any_user_to_be_co_host: bool
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    enum MeetingType {
        #[serde(rename = "meetingSeries")]
        MeetingSeries,

        #[serde(rename = "scheduledMeeting")]
        ScheduledMeeting,

        Meeting
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
        Expired
    }
}