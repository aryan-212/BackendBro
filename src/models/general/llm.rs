use dotenv::dotenv;
use reqwest::Client;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use serde_json::Value;
use std::env;
use std::error::Error;

pub async fn send_request(prompt: &str) -> Result<String, Box<dyn Error>> {
    dotenv().ok(); // Load .env file

    let api_key = env::var("GEMINI_API_KEY").map_err(|e| {
        eprintln!("Failed to get API key: {}", e);
        Box::new(e) as Box<dyn Error>
    })?;

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let client = Client::new();
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
        api_key
    );
    let body = serde_json::json!({
        "contents": [
            {
                "parts": [
                    {
                        "text": prompt
                    }
                ]
            }
        ]
    });

    let response = client
        .post(&url)
        .headers(headers)
        .json(&body)
        .send()
        .await
        .map_err(|e| {
            eprintln!("Request failed: {}", e);
            Box::new(e) as Box<dyn Error>
        })?;

    let status = response.status();
    let response_text = response
        .text()
        .await
        .map_err(|e| {
            eprintln!("Failed to read response text: {}", e);
            Box::new(e) as Box<dyn Error>
        })?
        .trim()
        .to_string();

    if !status.is_success() {
        eprintln!("API responded with error: {}", response_text);
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("API error: {}", response_text),
        )));
    }

    let parsed: Value = serde_json::from_str(&response_text).map_err(|e| {
        eprintln!("Failed to parse JSON: {}", e);
        Box::new(e) as Box<dyn Error>
    })?;

    if let Some(text) = parsed["candidates"]
        .get(0)
        .and_then(|c| c["content"]["parts"].get(0))
        .and_then(|p| p["text"].as_str())
    {
        Ok(text.to_string())
    } else {
        eprintln!("Unexpected API response format: {}", response_text);
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Cannot parse API response",
        )))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn testing_call() {
        match send_request("Is this working?").await {
            Ok(response) => println!("{:#?}", response),
            Err(e) => eprintln!("Test failed with error: {}", e),
        }
    }
}
