use crate::prompt::{build_documentation_request, get_url, send_request, GeminiResponse};
use bytes::Bytes;
use futures::StreamExt;
use std::env;
use std::io::{self, Write};

pub async fn generate_documentation(prompt: &str) -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set");
    let request_body = build_documentation_request(prompt);
    let url = get_url(&api_key, true);

    let response = send_request(&url, &request_body).await?;
    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk: Result<Bytes, reqwest::Error> = item;
        match chunk {
            Ok(chunk) => {
                let text_chunk = String::from_utf8_lossy(&chunk);
                for line in text_chunk.split('\n') {
                    let line = line.trim();
                    if line.starts_with("data: ") {
                        let data = line.strip_prefix("data: ").unwrap().trim();
                        if data == "[DONE]" {
                            return Ok(());
                        }
                        match serde_json::from_str::<GeminiResponse>(data) {
                            Ok(gemini_response) => {
                                if let Some(candidate) = gemini_response.candidates.get(0) {
                                    if let Some(part) = candidate.content.parts.get(0) {
                                        print!("{}", part.text);
                                        io::stdout().flush().unwrap();
                                    }
                                }
                            }
                            Err(e) => eprintln!("Error parsing JSON: {}", e),
                        }
                    }
                }
            }
            Err(e) => eprintln!("Error reading stream: {}", e),
        }
    }
    Ok(())
}
