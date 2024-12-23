use crate::error::Error;
use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    Json,
};
use base64::{engine::general_purpose, Engine as _};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use std::collections::HashMap;
use std::env;
use url::Url;

const ZOOM_OAUTH_URL: &str = "https://zoom.us/oauth/authorize";
const ZOOM_TOKEN_URL: &str = "https://zoom.us/oauth/token";
const API_MEETING_URL: &str = "https://api.zoom.us/v2/users/me/meetings";

fn zoom_oauth_url() -> Result<String, Error> {
    let client_id: String = env::var("client_id")?;
    let redirect_uri: String = env::var("redirect_uri")?;

    let mut url = Url::parse(ZOOM_OAUTH_URL)?;

    url.query_pairs_mut()
        .append_pair("response_type", "code")
        .append_pair("client_id", &client_id)
        .append_pair("redirect_uri", &redirect_uri);

    Ok(url.to_string())
}

fn base64_encode(client_id: &str, client_secret: &str) -> String {
    let credentials = format!("{}:{}", client_id, client_secret);
    general_purpose::STANDARD.encode(credentials.as_bytes())
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Token {
    pub access_token: String,
    pub token_type: String,
    pub refresh_token: String,
    pub expires_in: i32,
    pub scope: String,
    pub api_url: String,
}

async fn access_token(code: &str) -> Result<String, Error> {
    let client_id: String = env::var("client_id")?;
    let client_secret: String = env::var("client_secret")?;
    let redirect_uri: String = env::var("redirect_uri")?;

    let client = Client::new();

    let mut headers = HeaderMap::new();
    let base64_credentials = base64_encode(&client_id, &client_secret);
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Basic {}", base64_credentials))?,
    );

    let mut form_data = HashMap::new();
    form_data.insert("code", code);
    form_data.insert("grant_type", "authorization_code");
    form_data.insert("redirect_uri", &redirect_uri);

    let token = client
        .post(ZOOM_TOKEN_URL)
        .headers(headers)
        .form(&form_data)
        .send()
        .await?
        .json::<Token>()
        .await?;
    Ok(token.access_token)
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ZoomMeetingSettings {
    host_video: bool,
    participant_video: bool,
}

#[derive(Debug, serde::Serialize)]
struct ZoomMeeting {
    topic: String,
    #[serde(rename = "type")]
    meeting_type: i16,
    start_time: String,
    duration: i32,
    timezone: String,
    settings: ZoomMeetingSettings,
    // start_url: Option<String>,
    // join_url: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ZoomMeetingUrls {
    start_url: String,
    join_url: String,
}

async fn create_meeting(token: &str) -> Result<ZoomMeetingUrls, Error> {
    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", token))?,
    );

    let req = ZoomMeeting {
        topic: "topic".to_owned(),
        meeting_type: 2,
        start_time: "2024-12-25T10:00:00Z".to_owned(),
        duration: 30,
        timezone: "Asia/Tokyo".to_owned(),
        settings: ZoomMeetingSettings {
            host_video: true,
            participant_video: true,
        },
    };

    let resp = client
        .post(API_MEETING_URL)
        .headers(headers)
        .json(&req)
        .send()
        .await?
        .json::<ZoomMeetingUrls>()
        .await?;
    Ok(resp)
}

#[derive(Debug, serde::Deserialize)]
pub struct Code {
    code: Option<String>,
}

// TODO: so ugly, refactor using traits?
pub async fn zoom_auth(Query(Code { code }): Query<Code>) -> Response {
    match code {
        None => match zoom_oauth_url() {
            Ok(url) => Redirect::temporary(&url).into_response(),
            Err(err) => err.into_response(),
        },
        Some(code) => match access_token(&code).await {
            Ok(token) => match create_meeting(&token).await {
                Ok(urls) => (StatusCode::OK, Json(urls)).into_response(),
                Err(err) => err.into_response(),
            },
            Err(err) => err.into_response(),
        },
    }
}
