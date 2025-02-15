use crate::prompt::{build_command_request, get_url, send_request, GeminiResponse};
use reqwest::Error;
use std::env;

pub async fn generate_command(prompt: &str) -> Result<String, Error> {
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set");
    let request_body = build_command_request(prompt);
    let url = get_url(&api_key, false);

    let response = send_request(&url, &request_body).await?;
    let gemini_response: GeminiResponse = response.json().await?;

    let result = gemini_response
        .candidates
        .get(0)
        .and_then(|candidate| candidate.content.parts.get(0))
        .map(|part| part.text.clone())
        .unwrap_or_else(|| "No response received".to_string());

    Ok(result)
}
