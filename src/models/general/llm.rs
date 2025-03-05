use reqwest::{Client, header};
use std::env;
use std::error::Error;

pub async fn send_request(prompt: &str) -> Result<String, Box<dyn Error>> {
    let api_key = env::var("GEMINI_API_KEY")?;

    let mut headers = header::HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

    let client = Client::new();
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
        api_key
    );
    let body = format!(r#"{{"contents":[{{"parts":[{{"text":"{}"}}]}}]}}"#, prompt);

    let response = client
        .post(&url)
        .headers(headers)
        .body(body)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}
