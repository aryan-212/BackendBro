use dotenv::dotenv;
use reqwest::{Client, header};
use serde_json::Value;
use std::env;
use std::error::Error;

pub async fn send_request(prompt: &str) -> Result<String, Box<dyn Error>> {
    dotenv().ok(); // Load .env file

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
    let parsed: Value = serde_json::from_str(&response).unwrap();
    if let Some(text) = parsed["candidates"][0]["content"]["parts"][0]["text"].as_str() {
        return Ok(text.to_string());
    } else {
        return Ok("Cannot parse".to_string());
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use tokio;
    #[tokio::test]
    async fn testing_call() {
        let abc = send_request("Is this working?").await.unwrap();
        println!("{}", abc);
    }
}
